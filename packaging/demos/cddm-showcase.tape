# Charm VHS tape script for recording CDDM terminal workflows
# Usage: vhs packaging/demos/cddm-showcase.tape

Output docs/assets/cddm-cli-demo.gif
Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "TokyoNight"
Set Padding 20

Type "cddm --version"
Enter
Sleep 1s

Type "cddm scan . --min-tokens 50 --fail-threshold 5.0"
Enter
Sleep 3s

Type "cddm diff main --fail-threshold 3.0"
Enter
Sleep 2s

Type "cddm tui --help"
Enter
Sleep 2s
