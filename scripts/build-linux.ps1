param(
  [string]$OutDir = "binaries",
  [ValidateSet("all", "html2pdf", "weasyprint")]
  [string]$Target = "all"
)

$ErrorActionPreference = "Stop"
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $RepoRoot
try {
  New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
  if ($Target -eq "all") {
    docker build --platform=linux/amd64 --target export --output "type=local,dest=$OutDir" -f Dockerfile .
  } elseif ($Target -eq "html2pdf") {
    docker build --platform=linux/amd64 --target export --output "type=local,dest=$OutDir" -f packages/html2pdf/Dockerfile packages/html2pdf
  } else {
    docker build --platform=linux/amd64 --target export --output "type=local,dest=$OutDir" -f packages/weasyprint-freeze/Dockerfile packages/weasyprint-freeze
  }
  Write-Host "Exported Linux binaries to $OutDir"
} finally {
  Pop-Location
}
