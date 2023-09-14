#!/usr/bin/env bash

function compareVersions() {
    local newVersion=$1
    local currentVersion=$2

    newMajor=$(echo "$newVersion" | awk -F. '{ print $1 }' | sed 's/v//g')
    newMinor=$(echo "$newVersion" | awk -F. '{ print $2 }')
    newPatch=$(echo "$newVersion" | awk -F. '{ print $3 }')

    currentMajor=$(echo "$currentVersion" | awk -F. '{ print $1 }' | sed 's/v//g')
    currentMinor=$(echo "$currentVersion" | awk -F. '{ print $2 }')
    currentPatch=$(echo "$currentVersion" | awk -F. '{ print $3 }')

    if [ "$newMajor" -lt "$currentMajor" ]; then
      echo "Major version cannot decrease"
      exit 1
    elif [ "$newMinor" -lt "$currentMinor" ]; then
      echo "Minor version cannot decrease"
      exit 1
    elif [ "$newPatch" -lt "$currentPatch" ]; then
      echo "Patch version cannot decrease"
      exit 1
    fi
}

# Get version from arguments
version=$1

if [ -z "$version" ]; then
  echo "No version supplied - e.g. 0.1.0"
  exit 1
fi

# Get current version
currentVersion=$(grep '"version"' package.json | head -1 | awk -F: '{ print $2 }' | sed 's/[",]//g' | tr -d '[:space:]')

if [ -z "$currentVersion" ]; then
  echo "No current version found"
  exit 1
elif [ "$currentVersion" == "$version" ]; then
  echo "Version is already $version"
  exit 1
fi

compareVersions "$version" "$currentVersion"

os=$(uname -s)
if [ "$os" == "Darwin" ]; then
  sed -i '' 's/"version": "'"$currentVersion"'"/"version": "'"$version"'"/g' package.json
  sed -i '' 's/version = "'"$currentVersion"'"/version = "'"$version"'"/g' src-tauri/Cargo.toml
  sed -i '' 's/"version": "'"$currentVersion"'"/"version": "'"$version"'"/g' src-tauri/tauri.conf.json
elif [ "$os" == "Linux" ]; then
  sed -i 's/"version": "'"$currentVersion"'"/"version": "'"$version"'"/g' package.json
  sed -i 's/version = "'"$currentVersion"'"/version = "'"$version"'"/g' src-tauri/Cargo.toml
  sed -i 's/"version": "'"$currentVersion"'"/"version": "'"$version"'"/g' src-tauri/tauri.conf.json
fi

