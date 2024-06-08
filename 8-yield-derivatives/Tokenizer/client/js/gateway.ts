import { RadixDappToolkit, DataRequestBuilder, RadixNetwork, createLogger, NonFungibleIdType } from '@radixdlt/radix-dapp-toolkit'
import { it } from 'node:test';

const environment = process.env.NODE_ENV || 'Stokenet'; // Default to 'development' if NODE_ENV is not set
console.log("environment (gateway.js): ", environment)
// Define constants based on the environment
let dAppId, networkId, gwUrl: string, dashboardUrl: string;

if (environment == 'production') {
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Mainnet;
} else {
  // Default to Stokenet configuration
  dAppId = import.meta.env.VITE_DAPP_ID
  networkId = RadixNetwork.Stokenet;
}
gwUrl = import.meta.env.VITE_GATEWAY_URL;
dashboardUrl = import.meta.env.VITE_DASHBOARD_URL;
let component = import.meta.env.VITE_COMP_ADDRESS;
let userdata_nft = import.meta.env.VITE_USERDATA_NFT_RESOURCE_ADDRESS;
console.log("gw url (gateway.js): ", gwUrl)
console.log("dashboard url (gateway.js): ", dashboardUrl)
console.log("component address (gateway.js): ", component)

/**
 * Instantiate Radix Dapp Toolkit (RDT).
 * 
 */
export const rdt = RadixDappToolkit({
  dAppDefinitionAddress: dAppId,
  networkId: networkId,
  logger: createLogger(1),
  applicationName: 'Tokenizer',
  applicationVersion: '1.0.0'
  ,onDisconnect: () => {
    // clear your application state
    localStorage.removeItem('accountAddress')
  }
});

// Global states
let componentAddress = import.meta.env.VITE_COMP_ADDRESS //Component address on stokenet

// selected token // default XRD for Stokenet test
let selected_token = 'resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc';
// NFT data about the account 
// Be aware this is cached at the time the accounts logs in
// This cached data needs to be refreshed each time the account executes an operation
// This means that the cache expires after a supply, withdraw, tokenike and any other function
export let cached_user_position: any;

/**
 * Manage multi tokens by returning the token address based on the currency.
 */
export function getTokenAddress(currency: string) {
    console.log("getTokenAddress:", currency);
    if (currency === 'XRD') {
        let addr = 'resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc';
        selected_token = addr;
        return addr;
    } else if (currency === 'USDC') {
        let addr = 'resource_tdx_2_1t5e5q2jsn9eqe5ma0gqtpfjzqcmchjze28rfyttzunu3pr6y6t06t7';
        selected_token = addr;
        return addr;
    } else if (currency === 'HUG') {
        let addr = 'resource_tdx_2_1tkna28k99gnj24ngqxvrcl7dh7lrqyr85496guk2zgprjhg8nkvs5h';
        selected_token = addr;
        return addr;
    }  else if (currency === 'xWBTC') {
        let addr = 'resource_tdx_2_1t48fsfmh9kdfhdvge8x5dxee7u38wqcrjwesghjlks8lzmst725ccg';
        selected_token = addr;
        return addr;
    }  else if (currency === 'xETH') {
        let addr = 'resource_tdx_2_1tkjmydgvva5rl8x0lt9vn5lzpz2xh2d23klzjhv884hm9gg770l720';
        selected_token = addr;
        return addr;
    }
    // Return a default value or handle other cases as needed
    return '';
}

let accountAddress: string | null;

// ************ Fetch the user's account address (Page Load) ************
rdt.walletApi.setRequestData(DataRequestBuilder.accounts().atLeast(1))

// Subscribe to updates to the user's shared wallet data
const subscription = rdt.walletApi.walletData$.subscribe((walletData) => {
  accountAddress = walletData && walletData.accounts && walletData.accounts.length>0 ? walletData.accounts[0].address : null
  console.log("accountAddress : ", accountAddress)
  if (accountAddress!=null) {
    
    const element = document?.getElementById('accountAddress') as HTMLInputElement | null;
    if (element) {
        element.value = accountAddress ?? '';
    }

    // Store the accountAddress in localStorage
    localStorage.setItem('accountAddress', accountAddress);

    interface Hashmap {
      [key: string]: any;
    }    
    const hashmap: Hashmap = fetchComponentConfig(componentAddress)  
    //get config parameter of the component
    console.log("Hashmap:", hashmap);  

    //fetch nft metadata info of the connected user
    fetchUserPosition(accountAddress);
  }

})


// *********** Fetch Component Config (/state/entity/details) (Gateway) ***********
interface Hashmap {
  [key: string]: any;
}    
export async function fetchComponentConfig(_componentAddress: any): Promise<Hashmap>  {
  // Define the data to be sent in the POST request.
  const requestData = generatePayload("ComponentConfig", "", "Global");
  const hashmap: Hashmap = {};
  // Make an HTTP POST request to the gateway
  fetch(gwUrl+'/state/entity/details', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: requestData,
  })
  .then(response => response.json()) // Assuming the response is JSON data.
  .then(data => { 
    const json = data.items ? data.items[0] : null;
    
    const currentEpoch = data.ledger_state.epoch;

    const rewardValue = getReward(json);
    const extrarewardValue = getExtraReward(json);

    const currentRewardConfig = document.getElementById("currentReward");
    const currentExtraRewardConfig = document.getElementById("currentExtraReward");

    if (currentRewardConfig) currentRewardConfig.textContent = rewardValue + '%' ?? '';
    if (currentExtraRewardConfig) currentExtraRewardConfig.textContent = extrarewardValue + '%' ?? '';

  })
  .catch(error => {
      console.error('Error fetching data:', error);
  });
  return hashmap;
}


// ************ Utility Function (Gateway) *****************
function generatePayload(method: string, _address: string, resource_address: string) {
  let code;
  //console.log("generatePayload for method:", method);
  switch (method) {
    case 'ComponentConfig':
      console.log("generatePayload for method:", method);
      code = `{
        "addresses": [
          "${componentAddress}"
        ],
        "aggregation_level": "Global",
        "opt_ins": {
          "ancestor_identities": true,
          "component_royalty_vault_balance": true,
          "package_royalty_vault_balance": true,
          "non_fungible_include_nfids": true,
          "explicit_metadata": [
            "name",
            "description"
          ]
        }
      }`;
    break;   
    case 'UserPosition':
      console.log("generatePayload for method:", method);
      code = `{
        "addresses": [
          "${accountAddress}"
        ],
        "aggregation_level": "Vault",
        "opt_ins": {
          "ancestor_identities": true,
          "component_royalty_vault_balance": true,
          "package_royalty_vault_balance": true,
          "non_fungible_include_nfids": true,
          "explicit_metadata": [
            "name",
            "description"
          ]
        }
      }`;
    break;       
    // Add more cases as needed
    default:
      throw new Error(`Unsupported method: ${method}`);
  }
  return code;
}

// ************ Utility Function (Gateway) *****************
function getReward(data: { details: { state: { fields: any[]; }; }; }) {
  const rewardField = data.details.state.fields.find((field: { field_name: string; }) => field.field_name === "reward");
  return rewardField ? rewardField.value : null;
}

function getExtraReward(data: { details: { state: { fields: any[]; }; }; }) {
  const rewardField = data.details.state.fields.find((field: { field_name: string; }) => field.field_name === "extra_reward");
  return rewardField ? rewardField.value : null;
}


// *********** Fetch User NFT Metadata Information (/entity/details) (Gateway) ***********
export async function fetchUserPosition(_accountAddress: string) {
  // Define the data to be sent in the POST request.
  const requestData = generatePayload("UserPosition", "", "Vault");
  console.log("requestDat for entity/details:", requestData);

  // Make an HTTP POST request to the gateway
  fetch(gwUrl+'/state/entity/details', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: requestData,
  })
  .then(response => response.json()) // Assuming the response is JSON data.
  .then(data => { 
      const resourceAddress = `${userdata_nft}`;
      const result = getVaultsByResourceAddress(data, resourceAddress);
      console.log(" NFT id " + JSON.stringify(result));
      //TODO controllare la presenza di items
      const itemsArray = result && result.length>0 ? result[0].items : null
      console.log(" itemsArray " + itemsArray);
      // Loop through itemsArray and make GET requests for each item
      itemsArray?.forEach(async (item: any) => {
        await fetchNftMetadata(resourceAddress, item);
      });
  })
  .catch(error => {
      console.error('Error fetching data:', error);
  });
}



// *********** Fetch User NFT Metadata Information (Filtering response) (Gateway Utility) ***********
function getVaultsByResourceAddress(jsonData: { items: never[]; }, resourceAddress: string) {
  const items = jsonData.items || [];
  // Filter items based on the resource_address
  const filteredItems = items.filter((item: { non_fungible_resources: { items: any[]; }; }) => {
    return (
      item.non_fungible_resources &&
      item.non_fungible_resources.items &&
      item.non_fungible_resources.items.length > 0 &&
      item.non_fungible_resources.items.some(
        (        resource: { resource_address: any; }) =>
          resource.resource_address &&
          resource.resource_address === resourceAddress
      )
    );
  });

  // Extract vaults from the filtered items
  const vaults = filteredItems.reduce((result: any[], item: { non_fungible_resources: { items: any[]; }; }) => {
    if (
      item.non_fungible_resources &&
      item.non_fungible_resources.items &&
      item.non_fungible_resources.items.length > 0
    ) {
      const matchingResources = item.non_fungible_resources.items.filter(
        (        resource: { resource_address: any; }) =>
          resource.resource_address &&
          resource.resource_address === resourceAddress
      );
      
      matchingResources.forEach((resource: { vaults: { total_count: number; items: any; }; }) => {
        if (resource.vaults && resource.vaults.total_count > 0) {
          result.push(...resource.vaults.items);
        }
      });
    }
    return result;
  }, []);

  return vaults;
}



// *********** Fetch User NFT Metadata Information (/non-fungible/data) (Gateway Utility) ***********
async function fetchNftMetadata(resourceAddress: string, item: any) {
  // Define the data to be sent in the GET request.
  const requestData = `{
    "resource_address": "${resourceAddress}",
    "non_fungible_ids": [
      "${item}"
    ]
  }`;

  // Make an HTTP POST request to the gateway
  fetch(gwUrl+'/state/non-fungible/data', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: requestData,
  })
  .then(response => response.json()) 
  .then(data => { 
    //Inner function moved
    manageData(data);
    //In case you need something else you can do it here
    
  })
  .catch(error => {
      console.error('Error fetching data:', error);
  });
}

// *********** Read NFT (Gateway Utility) ***********
// This is for getting data out the NFT 
export function manageData(data: any) {
      cached_user_position = data;
      let token = selected_token;
      console.info('UserPosition, extract info about token = ', token);
      const extractedFields = extractFieldsFromJsonString(data, token);
  
      // Liquidity Data
      console.log(" amount ? " + extractedFields.amount);
      console.log(" epoch ? " + extractedFields.start_supply_epoch);
      console.log(" epoch ? " + extractedFields.end_supply_epoch);
      // Tokenized Data
      console.log(" tok. amount ? " + extractedFields.underlying_amount);
      console.log(" tok. extra ? " + extractedFields.extra_reward);
      console.log(" tok. claimed ? " + extractedFields.yield_claimed);
      console.log(" tok. maturity ? " + extractedFields.maturity_date);
      console.log(" tok. interest ? " + extractedFields.interest_totals);
      console.log(" tok. returned ? " + extractedFields.principal_returned);

      console.info('cached_user_position does exist ?  = ', cached_user_position);

      // Find the elements by their IDs
      const tokenSuppliedDiv = document.getElementById("tokenSupplied");
      const epochSuppliedDiv = document.getElementById("epochSupplied");
      const tokenLockedDiv = document.getElementById("tokenLocked");
      const epochLockedDiv = document.getElementById("epochLocked");
      const tokenExtraRewardRateDiv = document.getElementById("tokenExtraRewardRate");
      const tokenExtraRewardAmountDiv = document.getElementById("tokenExtraRewardAmount");
      // Update the content of the div elements about Your Position (Liquidity)
      tokenSuppliedDiv!.textContent = extractedFields.amount;
      epochSuppliedDiv!.textContent = extractedFields.start_supply_epoch;
      // Update the content of the div elements about Your Position (Tokenized)
      tokenLockedDiv!.textContent = extractedFields.underlying_amount;
      epochLockedDiv!.textContent = extractedFields.maturity_date;
      tokenExtraRewardRateDiv!.textContent = extractedFields.extra_reward;
      tokenExtraRewardAmountDiv!.textContent = extractedFields.interest_totals;

}

// Working with a simple String object
// Function to extract fields based on the target key
function extractFieldsFromJsonString(json: any, targetKey: string) {
  const result: { [key: string]: string } = {};

  if (json.non_fungible_ids && json.non_fungible_ids.length > 0) {
     // Directly access the first (and only) element
     // This needs to be checked  
      const nonFungible = json.non_fungible_ids[0];

      nonFungible.data.programmatic_json.fields.forEach((field: any) => {
          field.entries.forEach((entry: any) => {
              if (entry.key.value === targetKey) {
                  entry.value.fields.forEach((valueField: any) => {
                      result[valueField.field_name] = valueField.value;
                  });
              }
          });
      });
  } else {
      console.error("non_fungible_ids is undefined or empty");
  }

  return result;
}
