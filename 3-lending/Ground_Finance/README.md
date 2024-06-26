# Ground Finance: Make a Ground for your Web 3 Finance

Ground Finance is a blueprint package with 2 main usecases: on-chain credit service; on-chain lending protocol.

![logo](./GroundWeb/public/logo.svg)

## Current on-chain problem of uncollateralized lending protocols

Most on-chain lending protocol recent day cannot do uncollateralized lending, thus missed the 11 Tn market potential of uncollateralized loan. The problem came from the contradictory between "trust" characteristic of uncollateralized lending with on-chain "trustless" characteristic.

Some new projects are trying to challenge the problem through permissioned methods:

- [Aave](https://docs.aave.com/developers/guides/credit-delegation) credit delegation solution: push the "trust problem" to "lenders".
- [GoldFinch](https://docs.goldfinch.finance/goldfinch/goldfinch-overview) trust through consensus solution: solve the "trust problem" through a consensus from many permissioned entities (Backers, Auditors).
- [Centrifuge Tinlake](https://docs.centrifuge.io/getting-started/centrifuge-at-a-glance/) full permissioned solution for Investors, Issuers and Asset Originators, highly require off-chain "trust".
- [TrueFi](https://blog.trusttoken.com/introducing-truefi-the-defi-protocol-for-uncollateralized-lending-9bfd6594a48)  permissioned solution for institution borrowers, voted through by a DAO and provide a risk-backed solution for lenders.

Although Ground Finance also used permissioned solutions, it combined the best charateristic of these 4 uncollateralized lending solutions and evolved them into on-chain "consumer level" credit and "bank level" earning tracker while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.

## Quick Start

Clone this git repository: `git clone https://github.com/radixdlt/scrypto-challenges && cd 3-lending/Ground_Finance`

For the tests, this project use an extra test blueprint: [GroundTestEngine](./Ground_Test).

### Unit-test

The unit-test used the radix-engine; the community package scrypto_unit; cargo reqwest, tokio and serde_json.

1. Build the package: `cd Ground_Test && scrypto build`
2. Quick test: `scrypto test`
3. Study the [tests](./Ground_Test/tests/lib.rs) and test each function of the protocol.

### Public test

The public test used both the PTE resim client and the PTE Browser Extension, *the test will be running on <https://pte01.radixdlt.com/> sever*

The frontend is bootstraped with Vite and React.

You need a wallet address on the [PTE Browser Extension](https://docs.radixdlt.com/main/scrypto/public-test-environment/pte-getting-started.html) to participate on the public test.

**Test Component initializing and get testing resource through resim client:**

1. Connect to the PTE `cd Ground_Test/resim-client && resim-client --address pte01-socket.radixdlt.com:8010`
2. Check the test component `resim show 022b54ff498d7551bf97809e7a504a25631073f7635746af7ee644`. If the test component already existed, go directly to the step 8.
3. Close the resim client and build the test package `cd .. && scrypto build` (ignore this step if you have already dont the unit-test)
4. Connect to the PTE again `cd resim-client && resim-client --address pte01-socket.radixdlt.com:8010` and publish the test package `resim publish ../target/wasm32-unknown-unknown/release/ground_test.wasm`
5. Edit the package address on file [instantiate](./Ground_Test/resim-client/instantiate) and run `resim run instantiate`
6. Edit the output component address (Instruction Outputs:) on file [init](./Ground_Test/resim-client/init) and run `resim run init`
7. Edit all the component, resource address on file [GROUND_ADDRESS.tsx](./Ground_Test/resim-client/GROUND_ADDRESS.tsx) and replace the file into this [directory](./GroundWeb/src/assets/GROUND_ADDRESS.tsx)
8. Edit your account address on file [get_test_resources](./Ground_Test/resim-client/get_test_resources) and get the testing resources for your account by `resim run get_test_resources`.

**Frontend Public Test:**

1. run `cd GroundWeb && npm install`
2. run the UI `npm run dev`
3. Try the UI!

**Extra feature mean for testers on the resim client:**

- Manipulate time through the NeuRacle blueprint: re-check the address and edit variable on file [manipulate_time](./Ground_Test/resim-client/manipulate_time) and do `resim run manipulate_time`. (This must be called after at least an epoch or the NeuRacle component will panic because of the Sybil prevent function, if you see it panic because of the Sybil prevent function, it's most likely that you or some others has already run the manipulate time transaction on that epoch).

*This is only for test purpose and totally not the pratical use-case of the NeuRacle package!*

- Allow an installment credit through it's ID: re-check the address and edit variable on file [review_installment_credit](./Ground_Test/resim-client/review_installment_credit) and do `resim run review_installment_credit`

- Blacklist a credit user: re-check the address and edit variable on file [blacklist](./Ground_Test/resim-client/blacklist) and do `resim run blacklist`

- Change the credit scoring rates: re-check the address and edit variable on file [change_credit_scoring_rate](./Ground_Test/resim-client/change_credit_scoring_rate) and do `resim run change_credit_scoring_rate`

- Change the protocol's revolving credit interest rates: re-check the address and edit variable on file [change_interest_rates](./Ground_Test/resim-client/change_interest_rates) and do `resim run change_interest_rates`

- Change the protocol's withdrawal fee for lenders: re-check the address and edit variable on file [change_fee](./Ground_Test/resim-client/change_fee) and do `resim run change_fee`

- Change the protocol tolerance threshold: re-check the address and edit variable on file [change_tolerance_threshold](./Ground_Test/resim-client/change_tolerance_threshold) and do `resim run change_tolerance_threshold`

- Change the protocol's compensation rate: re-check the address and edit variable on file [change_compensate_rate](./Ground_Test/resim-client/change_compensate_rate) and do `resim run change_compensate_rate`

**Study more in the doc:**

Linux & Mac: `./doc.sh`

Windows (not have git bash): `cd Ground_Test && cargo doc --no-deps  --document-private-items --package ground_finance --package ground_test --open`

## [GroundCredit](./Ground_Finance/src/ground_credit.rs): Make a Credit Ground for your journey into Web 3

Ground Credit is the blueprint for any organization to help users build a credit Ground in Web3 Society by utilizing SBT characteristics.

Ground Credit also help lending protocol operators to use the Credit Service, allow automated credit scoring and debt-tracking through credit user's data.

### Main Features

The blueprint is for web3 organizations to manage user's credit through making use of Soul Bound Tokens (SBTs).

The blueprint utilized the Unique Identity Service from [GroundID](https://github.com/unghuuduc/GroundPackages/tree/main/Ground_ID) blueprint to assess unique identity on-chain, prevent Sybil attack.

The blueprint included installment type credit, allow [TrueFi](https://truefi.io/) level credit.

The blueprint also included two revolving credit types: "Monthly" and "Yearly", allow on-chain "consumer level" credit for borrowers.

### Protocol entities

1. **Credit service operator**: Main manager of the protocol. Through the blueprint's method, *Credit service operator* is allowed to:

- Issue new Credit SBT for users (for user who wish to migrate his off-chain credit history). (Require off-chain process)
- Review installment credit request. (Require off-chain process)
- List, delist a lending protocol to use the Credit service. (Require off-chain process if the credit service and the protocol weren't run by the same entity)
- Blacklist, whitelist credit users who have issue with the ID SBT (wrong income, trust score) or have a large loan default. (Require off-chain process)
<!-- - Change the credit degrade and restore rate when credit users have late (or on-time) repayment frequency. -->

Service operator is also required to protect user's private data.

2. **Credit users**: Verified unique identity on web3 who wish to use on-chain credit or take a loan. Through the blueprint's method, *Credit users* are allowed to:

- Use the ID SBT to take new credit SBT.
- Change credit type ("Monthly" or "Yearly") (Require no-debt credit status). The change will also reset the accumulated repaid amount on user's credit.
- Check the maximum credit and current credit allowance.
- Request an installment credit.
- Take the installment credit badge after the request has passed.

<!-- 3. **Lending protocols**: Listed lending protocols can use this blueprint for on-chain credit service. Through the blueprint's method, *Lending protocols* are allowed to:
- Automatically evaluate user's credit score through late (or on-time) repayment frequency. 
- Edit user's current debt or the credit's due time.
- Let protocol users use the installment credit badge to change credit into installment type (Require no-debt credit status).
- Let protocol users stop using installment credit and change the credit back into revolving type. (Now not working on Credit blueprint) -->

3. **Lending protocols**: Listed lending protocols can use this blueprint for on-chain credit service. Through the blueprint's method, *Lending protocols* are allowed to:

- Edit the Credit data and burn the Installment Credit Badge.

## [GroundLending](./Ground_Finance/src/ground_lending.rs): Make a Ground for your Web 3 Finance

Ground Lending is the core blueprint of the Ground Finance package, provide collateral-free lending solution to maximize capital efficiency for borrowers and earn rates for lenders, allow on-chain "bank level" earning tracker while protecting lender's privacy, ensuring security and dynamic, transparent interest rate at the same time.

### Main Features

The blueprint is for web3 organizations to instantiate and manage a collateral-free lending protocol on-chain.

The blueprint utilized the Credit Service from [GroundCredit](./Ground_Finance/src/ground_credit.rs) blueprint, the Oracle solution from [NeuRacle](https://github.com/unghuuduc/NeuRacle) blueprint
and the business DAO solution from [GroundBusinessDAO](https://github.com/unghuuduc/GroundPackages/blob/main/Ground_Business/src/ground_business_dao.rs) blueprint:

- The Credit Service is for the protocol to keep track and update the borrower's credit data: current debt (include initial debt, debt interest and extra debt by late repayment), credit score, credit due time, credit start time, accumulated repayment.

- The Oracle solution is for the protocol to keep track on the passage of time, to see which repayment is on-time (or late) and which lending accounts are eligible for the interest from borrowers, enable "bank level" earning tracker for lenders.

- The DAO solution is to run the protocol by collective actions, reduce human "bias" on the lending protocol.

The DAO also provide a "risk-backed" method called "compensate" which will compensate lenders a part of their lending, taken directly from the DAO treasury in case of cooperated loan defaults.

***Automated mechanism of the protocol:***

- Evaluate borrowers credit score through late (or on-time) repayment frequency.
- Edit user's current debt or the credit's due time when borrowers take loan or make a repayment
- Let borrowers use the installment credit badge to change their credit into installment type when taking the installment loan (Require no-debt credit status).
- Let borrowers stop using installment credit and change the credit back into revolving type when repaid all the installment loan.
- Apply lender's eligible interest after borrowers repay their interest.
- Deposit the lender's account fee or the borrower's extra debt to the fee vault (or directly into the DAO's treasury).

### Protocol Entities

1. **Protocol Operator**: Main manager of the protocol (can also be a DAO). Through the blueprint's method, *protocol operator* is allowed to:

- Change the DAO component address the protocol is using.
- Change the Oracle component address the protocol is using.
- Funding the Oracle account from a badge received from that Oracle.
- Change the protocol's revolving credit interest rates.
- Change the protocol's fee and compensate rate.
- Change the protocol's tolerance threshold (the minimum remained percent in protocol's vault allowed for borrowers to take a loan).
- Take the protocol's fee.
- Deposit a stable coin bucket into the protocol's vault to support the protocol in case of loan default.

2. **Lenders**: Any wallet address (permissionless) wish to lend the protocol their stable coin to maximize earn rates. Through the blueprint's method, *lenders* are allowed to:

- Lend an amount of stable coins into the protocol to earn interest and get the Account badge.
- Withdraw part of (or all) the return amount from the Account badge.
- Take the compensation from the DAO running this protocol in the worse case of cooperated loan default.

3. **Borrowers**: Permissioned wallet address (require ID SBT and Credit SBT) can make an automated collateral-free
loan through this blueprint to maximize capital efficiency.
Through the blueprint's method, *borrowers* are allowed to:

- Use the revolving credit SBT to take the revolving loan
- Use the installment credit badge to take the installment loan and change credit SBT into installment type. (require no-debt status)
- Get the current total debt (the debt is increased if user's late on repayment).
- Repay part of the current debt or repay in full.

## Security, Utility

### Dynamic credit types, enable "consumer-level" credit for borrowers

The Credit service blueprint has two credit type: "Revolving Credit" and "Installment Credit".

- Revolving Credit is allowed for any on-chain unique Identity with the income data (require an unique Identity SBT), maximum credit allowance will be calculated by a cubic function from 3 params: user's income, user's credit score, user's Identity trust score.

- Installment Credit is permissioned, only allowed for off-chain entity that likely need a legal procedure to protect the lending protocol from delinquent loan.

### Automatic credit scoring mechanism

The Ground Credit blueprint included an Automatic credit scoring mechanism:

- Credit user who is late on repayment will automatically get his credit score degraded.
- Credit user who has on-time repayment frequency and the total repayment reach the maximum allowance will get his credit score restored.

### "Bank level" earning tracker for lenders

Lenders can only earn the interest if their lending time on the protocol cover the **borrowers**'s borrowing time. Precisely, only when borrower borrow after a lender has lended their token on protocol and the lender won't withdraw the token until the borrower made repayment, that lender would earn the interest rate.

This is a "bank-like" utility that will incentive lenders to lend their money on the bank for a long time or they would not get the interest. This will also reduce mass-withdrawal risk from the protocol.

### Risk-tolerance mechanism

The Ground Lending blueprint included a Risk-tolerance mechanism, introduce a risk-tolerance threshold that prevent borrowers from getting a loan pass that risk-tolerance threshold.

Specifically, if the threshold is 60%, borrower cannot make a loan when there is only (or lower than) 60% asset left compared to the amount that protocol has to return for lenders.

### Risk-backed compensation through a DAO

Although it's permissionless for lenders, all borrowers require on-chain unique identity and thus have to use the [Ground ID](https://github.com/unghuuduc/GroundPackages/tree/main/Ground_ID) service, which converge into the centralization problem. Ground Finance cannot achive [Decentralized Credit](https://cointelegraph.com/news/decentralized-credit-scores-how-can-blockchain-tech-change-ratings) yet and vulnerable to "single point of failure" from the Identity service provider.

Even if the Ground Finance protocol and the Ground ID service are ran by the same DAO, bringing unique identity on-chain require human interaction (because identity is a "given" thing by other, like how our name are given by our parents) and thus still vulnerable to human "bias".

Confronting such risk, the protocol included a compensate method which utilized [GroundBusiness](https://github.com/unghuuduc/GroundPackages/tree/main/Ground_Business) package to back the protocol through a DAO.

## License

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
