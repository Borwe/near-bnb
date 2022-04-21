/**
 * Used for deploying contracts
 */

const sh = require('shelljs');
const prompt = require('prompt-sync')();
require('dotenv').config({ path: './neardev/dev-account.env' });
const colors = require('colors');


const main_contract = "./out/flats_factory.wasm"
const main_contract_address = process.env.MAIN_CONTRACT_ADDRESS;

if(main_contract_address===undefined || main_contract_address==='undefined'){
  console.log(colors.red("Error, couldn't find valid account address'"));
  console.log(colors.red("Please add: \"MAIN_CONTRACT_ADDRESS = \"contract_name\";\""));
}else{
  let main_owner = prompt("Set main contract owner, by passing in your account: ");
  console.log(colors.green("Creating contract account"));
  let command = `near create-account ${main_contract_address} --masterAccount ${main_owner}`
  sh.exec(command);
  console.log(colors.green("Ignore any errors, just means account already existed"));
  command = `near deploy ${main_contract_address} ${main_contract} new '{"owner":"${main_owner}"}'`;
  console.log(colors.green(`Going to run: ${command}`));
  let {result, errormsg} = 
    sh.exec(command);

  if(result==0){
    console.log(colors.green("Deployed contract succesfully, showing status"));
    sh.exec("near state ${main_contract_address}");
  }else{
    console.log(colors.red("If running this for second time after a succesful first run, then ignore this error"));
    console.log(colors.red("Error:"));
    console.log(colors.red(errormsg));
  }
}
