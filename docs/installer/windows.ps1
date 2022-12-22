
function Add-Path($Path) {
    $env:Path += ":" + $Path;
    $Path = [Environment]::GetEnvironmentVariable("PATH", "User") + [IO.Path]::PathSeparator + $Path
    [Environment]::SetEnvironmentVariable( "Path", $Path, "User")
}
$discloud_zip = New-TemporaryFile | Rename-Item -NewName { $_ -replace 'tmp', 'zip' } -PassThru
$out_dir = $env:APPDATA + "\discloud\"
"Downloading latest discloud CLI version"
(New-Object System.Net.WebClient).DownloadFile("https://github.com/discloud/cli-rust/releases/latest/download/discloud-x86_64-Windows.zip", $discloud_zip)
mkdir $out_dir -ea 0
"Extracting files"
Expand-Archive $discloud_zip -DestinationPath $out_dir -Force
if([Environment]::GetEnvironmentVariable("PATH", "User") -split ";" -ccontains $out_dir) {
    "Path is already setup correctly"
} else {
    Add-Path $out_dir
    "Path configured successfully"
}
"Cleaning up temporary files"
Remove-Item $discloud_zip.FullName -Force
"Done! You might want to restart your system to make the discloud cli available in the PATH"
"Troubleshooting: if it says that vcruntime140.dll is missing, try installing this: https://aka.ms/vs/17/release/vc_redist.x64.exe"