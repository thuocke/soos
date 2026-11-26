package io.github.thuocke.soos

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.sizeIn
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.BasicText
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.TextLayoutResult
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.style.LineHeightStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay

/** Shown in an empty page, and the shape every line follows. */
private const val CONVERTER_PLACEHOLDER = "1 lap (laps) = 400 m"

/**
 * The converters page: the user's own units, one per line, kept apart from
 * the documents. It reads like a document: what's typed on the left, and
 * beside each line what the unit is worth or why it can't be used.
 * soos-core reads the lines on every recalculation.
 */
@Composable
fun ConvertersScreen(state: AppState, palette: Palette, modifier: Modifier = Modifier) {
    val text = state.converters
    val cells = (state.calculated?.converters ?: emptyList()).map { status ->
        status?.let { Cell(it.ok ?: it.error ?: "", "", it.error, false) }
    }
    val font = mono()
    val style = remember(palette, font) {
        TextStyle(
            fontFamily = font,
            fontSize = FontSize,
            lineHeight = LineHeight,
            color = palette.plain,
            lineHeightStyle = LineHeightStyle(LineHeightStyle.Alignment.Center, LineHeightStyle.Trim.None),
        )
    }
    val highlighter = remember(palette) { Highlighter(palette) }
    val focus = remember { FocusRequester() }
    LaunchedEffect(Unit) { focus.requestFocus() }

    // Half a line is usually an error, so errors wait for a pause in typing.
    var edits by remember { mutableIntStateOf(0) }
    var typing by remember { mutableStateOf(false) }
    LaunchedEffect(edits) {
        if (edits > 0) {
            typing = true
            delay(ERROR_HOLD_MS)
            typing = false
        }
    }

    val charWidth = with(LocalDensity.current) { (FontSize * CharWidthEm).toDp() }
    val lineStarts = remember(text) {
        buildList {
            add(0)
            text.forEachIndexed { i, c -> if (c == '\n') add(i + 1) }
        }
    }
    var layout by remember { mutableStateOf<TextLayoutResult?>(null) }

    Column(modifier.fillMaxSize()) {
        Row(
            Modifier.padding(start = PagePad, end = PagePad / 2),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            BasicText(
                "Your converters",
                style = textStyle(palette.plain).copy(fontWeight = FontWeight.Bold),
                modifier = Modifier.weight(1f),
            )
            Box(
                Modifier
                    .sizeIn(minWidth = 48.dp, minHeight = 48.dp)
                    .clickable(role = Role.Button, onClickLabel = "Close the converters") {
                        state.showConverters = false
                    }
                    .padding(horizontal = 8.dp),
                contentAlignment = Alignment.Center,
            ) {
                BasicText("Done", style = textStyle(palette.keyword))
            }
        }
        BasicText(
            "One unit per line.",
            style = textStyle(palette.comment),
            modifier = Modifier.padding(horizontal = PagePad),
        )
        BoxWithConstraints(Modifier.weight(1f).fillMaxWidth()) {
            val gutter = gutterWidth(cells, maxWidth - PagePad * 2, charWidth)
            val viewport = maxHeight
            Box(
                Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(horizontal = PagePad, vertical = 12.dp),
            ) {
                BasicTextField(
                    value = text,
                    onValueChange = {
                        edits += 1
                        state.editConverters(it)
                    },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(end = gutter + ColumnGap)
                        // A tap below the text still places the caret.
                        .defaultMinSize(minHeight = viewport - 24.dp)
                        .focusRequester(focus),
                    textStyle = style,
                    cursorBrush = SolidColor(palette.plain),
                    visualTransformation = highlighter,
                    keyboardOptions = KeyboardOptions(
                        capitalization = KeyboardCapitalization.None,
                        autoCorrectEnabled = false,
                    ),
                    onTextLayout = { layout = it },
                    decorationBox = { field ->
                        Box {
                            if (text.isEmpty()) {
                                BasicText(CONVERTER_PLACEHOLDER, style = style.copy(color = palette.comment))
                            }
                            field()
                        }
                    },
                )

                val measured = layout
                if (measured != null) {
                    cells.forEachIndexed { line, cell ->
                        if (cell == null || line >= lineStarts.size) return@forEachIndexed
                        val error = cell.error
                        if (error != null && typing) return@forEachIndexed
                        val offset = lineStarts[line].coerceAtMost(measured.layoutInput.text.length)
                        val top = measured.getLineTop(measured.getLineForOffset(offset))
                        BasicText(
                            text = cell.text,
                            style = style.copy(
                                color = if (error != null) palette.error else palette.result,
                                textAlign = TextAlign.End,
                            ),
                            maxLines = 1,
                            softWrap = false,
                            overflow = TextOverflow.Ellipsis,
                            modifier = Modifier
                                .align(Alignment.TopEnd)
                                .offset { IntOffset(0, top.toInt()) }
                                .width(gutter)
                                // The column cuts a long error; a tap shows all of it.
                                .then(
                                    if (error != null) {
                                        Modifier.clickable(role = Role.Button, onClickLabel = "Show the error") {
                                            state.show(error, error = true, forMillis = 4000)
                                        }
                                    } else {
                                        Modifier
                                    },
                                ),
                        )
                    }
                }
            }
        }
    }
}
