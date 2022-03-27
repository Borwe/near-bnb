// This file does two things:
//
// 1. Compile the Rust contract using cargo (see buildCmd below). This will
//    create a wasm file in the 'build' folder.
// 2. Create a symbolic link (symlink) to the generated wasm file in the root
//    project's `out` folder, for easy use with near-cli.
//
// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require('shelljs')

// Figure out which directory the user called this script from, which we'll use
// later to set up the symlink.
const calledFromDir = sh.pwd().toString()

// For the duration of this script, we want to operate from within the
// Rust project's folder. Let's change into that directory.
sh.cd(__dirname)

// You can call this script with `node compile.js` or `node compile.js
// --debug`. Let's set a variable to track whether `--debug` was used.
const debug = process.argv.pop() === '--debug'

const buildFlatFactoryCmd = debug
  ? 'cargo build --target wasm32-unknown-unknown -p flats_factory'
  : 'cargo build --target wasm32-unknown-unknown -p flats_factory --release'

const buildFlatContractCmd = debug
  ? 'cargo build --target wasm32-unknown-unknown -p flats_contract'
  : 'cargo build --target wasm32-unknown-unknown -p flats_contract --release'

const { code } = sh.exec(buildFlatContractCmd)


if (code === 0 && calledFromDir !== __dirname) {
  const linkDir = `${calledFromDir}/out`
  const flats_wasms = ["flats_contract", "flats_factory"];
  sh.mkdir('-p', linkDir)
  for(let i = 0; i<flats_wasms.length;i++){
    const packageName = flats_wasms[i];
    const outFile = `./target/wasm32-unknown-unknown/${debug ? 'debug' : 'release'}/${packageName}.wasm`
    const link = `${calledFromDir}/out/${packageName}.wasm`
    sh.rm('-f', link)

    if(packageName=="flats_contract"){
      sh.cp('-u',outFile,link)
    }else{
      if(sh.exec(buildFlatFactoryCmd)==0){
        sh.cp('-u',outFile,link)
      }else{
        console.error("Error building flat_factory")
      }
    }
  }
}

// exit script with the same code as the build command
process.exit(code)
