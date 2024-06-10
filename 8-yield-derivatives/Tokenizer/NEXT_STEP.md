# Tokenizer

This application is to apply for the next Booster Grant available (5000$).



# Improvement

This is the minimum list of improvements needed to apply for the grant.


## Improvement (Major)

Scrypto
- Current Reward for suppliers needs to be token-specific (now it is the same for all the tokens)
- Current Extra-Reward for tokenizers needs to be token-specific (now it is the same for all the tokens)

Architecture
- Setup the new OCI library to fetching interest rate changes for then storing in a database
- Setup a new API (Kotlin) for serving the interest rate values (filtered by start/end epoch and resource address)
- Setup a new frontend library for showing the graph about the interest rate movement

Platform Panel
- (Frontend) Show tabular info about supplied tokens amount
- (Frontend) Show graph info about locked tokens (Create and event during tokenize, store data and create API)

Frontend
- 'Position Summary' and 'Operational Panel' refinement
- 'Epoch' should be converted from 'epochs' to a date
- 'Operational Panel' -> Clicking on the token should highligth the corresponding line in the table below
- PopUp for each function -> Clicking on 'Supply' or 'Tokenized' or any other button should open a PopUp (//mock not ready yet)
- 'Position Summary' should highlight amount of tokens supplied and locked

Terraform
- Copy terraform script for updating the website

Business
- Study Pendle whitepaper and prepare a Tokenizer whitepaper

## Improvement (Medium)

Architecture
- how to move dinamically the reward/extra reward (//not ready) based on amount supplied/locked

## Improvement (Minor)

- Suppliers should be able to add liquidity without withdrawing first

## Improvement (Future)

Architecture
- Create a function for Trade a locked position (This means burn the PT and YT and return a TKN-USDC for example, that is withdrawable directly from the platform)
- Create a function for Trade a locked indipendently a PT or a YT position 
- Create an Order Book for trading PT or YT separately

# Current Bug

- Extra amount coming from the yield claimed is not updated back as 'Liquidity Data' 
(This means that this amount remain locked in the contract)


# Let's finally have a look at the Online demo dApp 

You can also try the deployed dApp here https://zerocollateral.eu/

# License

The Radix Scrypto Challenges code is released under Radix Modified MIT License.

Copyright 2024 Radix Publishing Ltd

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software for non-production informational and educational purposes without
restriction, including without limitation the rights to use, copy, modify,
merge, publish, distribute, sublicense, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

This notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE HAS BEEN CREATED AND IS PROVIDED FOR NON-PRODUCTION, INFORMATIONAL
AND EDUCATIONAL PURPOSES ONLY.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE, ERROR-FREE PERFORMANCE AND NONINFRINGEMENT. IN NO
EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES,
COSTS OR OTHER LIABILITY OF ANY NATURE WHATSOEVER, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE, MISUSE OR OTHER DEALINGS IN THE SOFTWARE. THE AUTHORS SHALL
OWE NO DUTY OF CARE OR FIDUCIARY DUTIES TO USERS OF THE SOFTWARE.