package io.github.thuocke.soos

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.sizeIn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.BasicText
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

/**
 * The settings list, as on the desktop: one row per setting, its name on the
 * left and what it is now on the right, so no state is told by colour or by
 * a symbol.
 */
@Composable
fun SettingsScreen(state: AppState, palette: Palette, modifier: Modifier = Modifier) {
    val units = state.calculated?.converters?.count { it?.ok != null } ?: 0
    Column(
        modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(start = 16.dp, end = 8.dp),
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            BasicText(
                "Settings",
                style = textStyle(palette.plain).copy(fontWeight = FontWeight.Bold),
                modifier = Modifier.weight(1f),
            )
            Box(
                Modifier
                    .sizeIn(minWidth = 48.dp, minHeight = 48.dp)
                    .clickable(role = Role.Button, onClickLabel = "Close the settings") {
                        state.showSettings = false
                    }
                    .padding(horizontal = 8.dp),
                contentAlignment = Alignment.Center,
            ) {
                BasicText("Done", style = textStyle(palette.keyword))
            }
        }
        // The tab strip only shows from the second tab on, so this is the way to it.
        if (state.tabs.size < MAX_TABS) {
            SettingRow("New tab", "+", palette) {
                state.addTab()
                state.showSettings = false
            }
        }
        SettingRow(
            "Currency amounts",
            if (state.highPrecision) "every digit" else "rounded",
            palette,
        ) { state.toggleHighPrecision() }
        SettingRow(
            "Your converters",
            when (units) {
                0 -> "none yet"
                1 -> "1 unit"
                else -> "$units units"
            },
            palette,
        ) {
            state.showSettings = false
            state.showConverters = true
        }
        SettingRow("Version", state.version, palette, onClick = null)
    }
}

/** One setting: `label`, then its current `value`. Without `onClick` it only shows. */
@Composable
private fun SettingRow(label: String, value: String, palette: Palette, onClick: (() -> Unit)?) {
    Row(
        Modifier
            .fillMaxWidth()
            .heightIn(min = 48.dp)
            .then(if (onClick != null) Modifier.clickable(role = Role.Button, onClick = onClick) else Modifier)
            .padding(end = 8.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        BasicText(label, style = textStyle(palette.plain), modifier = Modifier.weight(1f))
        BasicText(value, style = textStyle(if (onClick != null) palette.keyword else palette.comment))
    }
}
