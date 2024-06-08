// Import the variable from your JavaScript file
import { getTokenAddress, manageData, cached_user_position } from './gateway.ts';

// Get a reference to the select element
const currencySelect = document.getElementById('currencySelect');
console.log(`does it exist  ${currencySelect} `);

currencySelect.addEventListener('change', function() {
    // Call the function to get the updated tokenAddress based on the selected value
    let newTokenAddress = getTokenAddress(currencySelect.value);
    console.log(`current token address ${newTokenAddress} `);
    // set the data from the NFT in the corresponding section in the web page
    manageData(cached_user_position);
});
