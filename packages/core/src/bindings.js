/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* NAPI-RS bindings loader for @opencode-cloud/core */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'core.darwin-arm64.node'))
    if (arch === 'arm64') {
      try {
        if (localFileExisted) {
          nativeBinding = require('./core.darwin-arm64.node')
        } else {
          nativeBinding = require('@opencode-cloud/core-darwin-arm64')
        }
      } catch (e) {
        loadError = e
      }
    } else {
      localFileExisted = existsSync(join(__dirname, 'core.darwin-x64.node'))
      try {
        if (localFileExisted) {
          nativeBinding = require('./core.darwin-x64.node')
        } else {
          nativeBinding = require('@opencode-cloud/core-darwin-x64')
        }
      } catch (e) {
        loadError = e
      }
    }
    break
  case 'linux':
    if (arch === 'x64') {
      if (isMusl()) {
        localFileExisted = existsSync(join(__dirname, 'core.linux-x64-musl.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./core.linux-x64-musl.node')
          } else {
            nativeBinding = require('@opencode-cloud/core-linux-x64-musl')
          }
        } catch (e) {
          loadError = e
        }
      } else {
        localFileExisted = existsSync(join(__dirname, 'core.linux-x64-gnu.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./core.linux-x64-gnu.node')
          } else {
            nativeBinding = require('@opencode-cloud/core-linux-x64-gnu')
          }
        } catch (e) {
          loadError = e
        }
      }
    } else if (arch === 'arm64') {
      if (isMusl()) {
        localFileExisted = existsSync(join(__dirname, 'core.linux-arm64-musl.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./core.linux-arm64-musl.node')
          } else {
            nativeBinding = require('@opencode-cloud/core-linux-arm64-musl')
          }
        } catch (e) {
          loadError = e
        }
      } else {
        localFileExisted = existsSync(join(__dirname, 'core.linux-arm64-gnu.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./core.linux-arm64-gnu.node')
          } else {
            nativeBinding = require('@opencode-cloud/core-linux-arm64-gnu')
          }
        } catch (e) {
          loadError = e
        }
      }
    }
    break
  case 'win32':
    if (arch === 'x64') {
      localFileExisted = existsSync(join(__dirname, 'core.win32-x64-msvc.node'))
      try {
        if (localFileExisted) {
          nativeBinding = require('./core.win32-x64-msvc.node')
        } else {
          nativeBinding = require('@opencode-cloud/core-win32-x64-msvc')
        }
      } catch (e) {
        loadError = e
      }
    } else if (arch === 'arm64') {
      localFileExisted = existsSync(join(__dirname, 'core.win32-arm64-msvc.node'))
      try {
        if (localFileExisted) {
          nativeBinding = require('./core.win32-arm64-msvc.node')
        } else {
          nativeBinding = require('@opencode-cloud/core-win32-arm64-msvc')
        }
      } catch (e) {
        loadError = e
      }
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { getVersionJs, getVersionLongJs } = nativeBinding

module.exports.getVersionJs = getVersionJs
module.exports.getVersionLongJs = getVersionLongJs
