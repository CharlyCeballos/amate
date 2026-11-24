# Windows UI Automation probe: what does a screen reader get from the editor (AccessKit -> UIA)?
# Usage (Windows PowerShell 5.1): powershell -NoProfile -File a11y-probe.ps1 [-File path]
# Launches the release build, prints what UIA exposes, then stops the app. Never sends keys.
param([string]$File = "$PSScriptRoot\samples\scripts.txt")
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$exe = Join-Path $PSScriptRoot 'target\release\amate-spike.exe'
$app = Start-Process -FilePath $exe -ArgumentList ('"{0}"' -f $File) -PassThru
# Step 5 can crash this (.NET) UIA client; a separate process still stops the app afterwards.
Start-Process -FilePath powershell -WindowStyle Hidden -ArgumentList @(
    '-NoProfile', '-Command', ('Start-Sleep -Seconds 30; Get-Process -Id {0} -ErrorAction SilentlyContinue | Where-Object {{ $_.Path -eq ''{1}'' }} | Stop-Process -Force' -f $app.Id, $exe))
$T = [System.Windows.Automation.Text.TextUnit]
$EP = [System.Windows.Automation.Text.TextPatternRangeEndpoint]
try {
    $root = [System.Windows.Automation.AutomationElement]::RootElement
    $byPid = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty, $app.Id)
    $editCond = New-Object System.Windows.Automation.PropertyCondition(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Edit)
    $editor = $null
    for ($i = 0; $i -lt 40 -and -not $editor; $i++) {
        Start-Sleep -Milliseconds 250
        $window = $root.FindFirst([System.Windows.Automation.TreeScope]::Children, $byPid)
        if ($window) { $editor = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $editCond) }
    }
    if (-not $editor) { Write-Output 'RESULT: no Edit element found'; exit 1 }
    $current = $editor.Current
    Write-Output ("1. ControlType: {0}; Name: {1}; HasKeyboardFocus: {2}" -f
        $current.ControlType.ProgrammaticName, $current.Name, $current.HasKeyboardFocus)
    Write-Output ("   Patterns: {0}" -f (($editor.GetSupportedPatterns() | ForEach-Object { $_.ProgrammaticName }) -join ', '))
    $valuePattern = $editor.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    Write-Output ("2. ValuePattern (our set_value, first 600 chars):\`n{0}" -f $valuePattern.Current.Value.Substring(0, [math]::Min(600, $valuePattern.Current.Value.Length)))
    $textPattern = $editor.GetCurrentPattern([System.Windows.Automation.TextPattern]::Pattern)
    Write-Output ("3. TextPattern (egui text runs, first 600 chars):\`n{0}" -f $textPattern.DocumentRange.GetText(600))
    foreach ($unit in @($T::Character, $T::Word, $T::Line)) {
        try {
            $range = $textPattern.DocumentRange.Clone()
            $range.MoveEndpointByRange($EP::End, $range, $EP::Start)
            [void]$range.Move($T::Line, 2)
            $range.ExpandToEnclosingUnit($unit)
            Write-Output ("4. Third line, expand {0}: '{1}'" -f $unit, $range.GetText(120).TrimEnd())
        } catch { Write-Output ("4. Third line, expand {0}: FAILED {1}" -f $unit, $_.Exception.Message) }
    }
    $selection = $textPattern.GetSelection()[0]
    Write-Output ("5. Caret (GetSelection) at document start: {0}" -f
        ($selection.CompareEndpoints($EP::Start, $textPattern.DocumentRange, $EP::Start) -eq 0))
    try {
        $selection.ExpandToEnclosingUnit($T::Line)
        Write-Output ("   Caret range expand Line: '{0}'" -f $selection.GetText(120).TrimEnd())
    } catch { Write-Output ("   Caret range expand Line: FAILED {0}" -f $_.Exception.Message) }
} finally {
    Stop-Process -Id $app.Id -Force -ErrorAction SilentlyContinue
}
