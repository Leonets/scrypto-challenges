use scrypto::prelude::*;
use scrypto_avltree::AvlTree;
use crate::utils::*;

// NFT to manage account position
#[derive(ScryptoSbor, NonFungibleData)]
pub struct UserMultiPosition {
    #[mutable]
    liquidity_position: HashMap<ResourceAddress, LiquidityData>,
    #[mutable]
    yield_token_data: HashMap<ResourceAddress, YieldTokenData>
}

// to contain data about account's provided liquidity 
#[derive(Copy, Clone, ScryptoSbor, NonFungibleData)]
 pub struct LiquidityData {
    #[mutable]
    start_supply_epoch: Epoch,
    #[mutable]
    end_supply_epoch: Epoch,
    #[mutable]
    amount: Decimal,
}

// to contain data about account's tokenized liquidity 
#[derive(Copy, Clone, ScryptoSbor, NonFungibleData)]
pub struct YieldTokenData {
    extra_reward: Decimal,
    underlying_amount: Decimal,
    pub interest_totals: Decimal,
    pub yield_claimed: Decimal,
    maturity_date: Decimal,
    principal_returned: bool,
}


#[derive(NonFungibleData, ScryptoSbor)]
struct StaffBadge {
    username: String
}

// Define the Operation enum
#[derive(Debug, Encode, Describe, Decode)]
pub enum Operation {
    Supply,
    TakesBack,
    Borrow,
    Repay,
    Redeem,
    Tokenize,
    Claim,
    Swap,
    No,
}

#[blueprint]
mod tokenizer {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            staff => updatable_by: [admin, OWNER];
        },
        methods {
            register => PUBLIC;
            unregister => PUBLIC;
            extra => PUBLIC;
            extra_supply => PUBLIC;
            supply => PUBLIC;
            takes_back => PUBLIC;
            set_reward => restrict_to: [staff, admin, OWNER];
            set_extra_reward => restrict_to: [admin, OWNER];
            set_reward_type => restrict_to: [admin, OWNER];
            extend_lending_pool => restrict_to: [admin, OWNER];
            fund_main_pool => restrict_to: [admin, OWNER];
            config => restrict_to: [staff, admin, OWNER];
            add_token => restrict_to: [admin, OWNER];
            mint_staff_badge => restrict_to: [staff, admin, OWNER];
            tokenize_yield  => PUBLIC;
            redeem => PUBLIC;
            redeem_from_pt => PUBLIC;
            claim_yield => PUBLIC;
        }
    }
    struct Tokenizer<> {
        tokenizer_vault: Vault,
        collected: HashMap<ResourceAddress, FungibleVault>,
        reward: Decimal,
        extra_reward: Decimal,
        tokenize_epoch_max_lenght: Decimal,
        tokenizer_manager: ResourceManager,
        nft_manager: ResourceManager,
        reward_type: String,
        interest_for_suppliers: AvlTree<Decimal, Decimal>,
        min_loan_limit: Decimal,
        max_loan_limit: Decimal,
        staff: AvlTree<u16, NonFungibleLocalId>,
        pt_resource_manager: ResourceManager,
        staff_badge_resource_manager: ResourceManager
        ,resource_a: ResourceAddress
        ,resource_b: ResourceAddress
    }

    impl Tokenizer {
        // given a reward level,symbol name, reward_type creates a ready-to-use Tokenizer
        pub fn instantiate(
            reward: Decimal,
            symbol: String,
            reward_type: String
            ,resource_a: ResourceAddress
            ,resource_b: ResourceAddress
        ) -> (Global<Tokenizer>, FungibleBucket, FungibleBucket) {

            let mut collected: HashMap<ResourceAddress, FungibleVault> = HashMap::new();
            collected.insert(resource_a, FungibleVault::new(resource_a));
            collected.insert(resource_b, FungibleVault::new(resource_b));
            
            //data struct for holding interest levels for loan
            let mut lend_tree: AvlTree<Decimal, Decimal> = AvlTree::new();
            lend_tree.insert(Decimal::from(Runtime::current_epoch().number()), reward);
            //staff
            let staff: AvlTree<u16, NonFungibleLocalId> = AvlTree::new();

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(Tokenizer::blueprint_id());

            let owner_badge = 
                ResourceBuilder::new_fungible(OwnerRole::None)
                    .metadata(metadata!(init{
                        "name"=>"Tokenizer Owner badge", locked;
                        "symbol" => "Tokenizer Owner", locked;
                        "description" => "A badge to be used for some extra-special administrative function", locked;
                    }))
                    .divisibility(DIVISIBILITY_NONE)
                    .mint_initial_supply(1);

            let admin_badge = 
                ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                .metadata(metadata!(init{
                    "name"=>"Tokenizer Admin badge", locked;
                    "symbol" => "Tokenizer Admin", locked;
                    "description" => "A badge to be used for some special administrative function", locked;
                }))
                .mint_roles(mint_roles! (
                        minter => rule!(require(global_caller(component_address)));
                        minter_updater => OWNER;
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);


            // create a new resource, with a fixed quantity of 100000
            let tokenizer_bucket = 
                ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(init{
                    "name" => "LiquidTokenizerUnit", locked;
                    "symbol" => symbol, locked;
                    "description" => "A token to use to receive back the loan", locked;
                }))
                .mint_roles(mint_roles! (
                         minter => rule!(require(global_caller(component_address)));
                         minter_updater => OWNER;
                ))
                .mint_initial_supply(100000);

            // Create a resourceManager to manage NFT
            let nft_manager =
                ResourceBuilder::new_ruid_non_fungible::<UserMultiPosition>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(
                    init {
                        "name" => "Tokenizer AccountData NFT", locked;
                        "symbol" => "Tokenizer AccountData", locked;
                        "description" => "An NFT containing information about your liquidity", locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(require(global_caller(component_address)));
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => OWNER;
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => OWNER;
                ))           
                .create_with_no_initial_supply();
       

            // Create a resourceManager to manage Principal Token
            let pt_rm: ResourceManager = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(
                require(owner_badge.resource_address())
                    || require(admin_badge.resource_address())
            )))
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata! {
                    init {
                        "name" => "Principal Token", locked;
                        "symbol" => "PT", locked;
                        "description" => "A Token containing the Principal Token", locked;
                        "yield_tokenizer_component" => GlobalAddress::from(component_address), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(allow_all);
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Create a resourceManager to manage Yield NonFungibleToken
            let staff_resource_manager: ResourceManager = 
                ResourceBuilder::new_ruid_non_fungible::<StaffBadge>(OwnerRole::Updatable(rule!(
                    require(owner_badge.resource_address())
                        || require(admin_badge.resource_address())
                )))
                .metadata(metadata!(init{
                    "name" => "Tokenizer Staff_badge", locked;
                    "symbol" => "Tokenizer Staff", locked;
                    "description" => "A badge to be used for some administrative function", locked;
                }))
                .mint_roles(mint_roles! (
                         minter => rule!(require(global_caller(component_address)));
                         minter_updater => OWNER;
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(require(admin_badge.resource_address()));
                    burner_updater => OWNER;
                ))
                .recall_roles(recall_roles! {
                    recaller => rule!(require(admin_badge.resource_address()));
                    recaller_updater => OWNER;
                })
            .create_with_no_initial_supply();            

            // populate a Tokenizer struct and instantiate a new component
            let component = 
                Self {
                    tokenizer_manager: tokenizer_bucket.resource_manager(),
                    tokenizer_vault: Vault::with_bucket(tokenizer_bucket.into()),
                    reward: reward,
                    extra_reward: dec!(10),
                    tokenize_epoch_max_lenght: dec!(518000),//how many days ??
                    nft_manager: nft_manager,
                    reward_type: reward_type,
                    interest_for_suppliers: lend_tree,
                    min_loan_limit: dec!(1),
                    max_loan_limit: dec!(10001),
                    staff: staff,
                    pt_resource_manager: pt_rm,
                    staff_badge_resource_manager: staff_resource_manager,
                    collected: collected
                    ,resource_a: resource_a
                    ,resource_b: resource_b
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(
                    owner_badge.resource_address()
                ))))
                .enable_component_royalties(component_royalties! {
                    // The roles section is optional, if missing, all roles default to OWNER
                    roles {
                        royalty_setter => rule!(allow_all);
                        royalty_setter_updater => OWNER;
                        royalty_locker => OWNER;
                        royalty_locker_updater => rule!(deny_all);
                        royalty_claimer => OWNER;
                        royalty_claimer_updater => rule!(deny_all);
                    },
                    init {
                        register => Free, locked;
                        unregister => Free, locked;
                        extra => Xrd(10.into()), updatable;
                        extra_supply => Free, locked;
                        supply => Xrd(10.into()), updatable;
                        takes_back => Xrd(10.into()), updatable;

                        set_reward => Free, locked;
                        set_extra_reward => Free, locked;
                        set_reward_type => Free, locked;
                        extend_lending_pool => Free, locked;
                        fund_main_pool => Free, locked;
                        config => Free, locked;
                        add_token => Free, locked;
                        mint_staff_badge => Free, locked;

                        tokenize_yield => Xrd(10.into()), updatable;
                        redeem => Xrd(10.into()), updatable;
                        redeem_from_pt => Xrd(10.into()), updatable;
                        claim_yield => Xrd(10.into()), updatable;
                    }
                })                
                .metadata(metadata!(
                    init {
                        "name" => "Tokenizer", locked;
                        "icon_url" => Url::of("https://test.Tokenizer.eu/images/logo3b.jpg"), locked;
                        "description" => "Tokenizer SmartContract for lending and tokenizer service", locked;
                        "claimed_websites" =>  ["https://test.Tokenizer.eu"], locked;
                    }
                ))//specify what this roles means
                .roles(roles!(
                    admin => rule!(require(admin_badge.resource_address()));
                    staff => rule!(require(staff_resource_manager.address()));
                ))
                .with_address(address_reservation)
                .globalize();
 
            return (component, admin_badge, owner_badge);
        }

        //add new token resource address
        pub fn add_token(&mut self, token: ResourceAddress)  {
            info!("Adding token type {:?}", token);
            self.collected.insert(token, FungibleVault::new(token));
        }

        //register to the platform
        pub fn register(&mut self) -> NonFungibleBucket {
            //mint an NFT for registering loan amount and starting/ending epoch
            let yield_token = self.init_yield();
            let liq_data = self.init_liq_data();

            let mut nft1: HashMap<ResourceAddress, LiquidityData> = HashMap::new();
            nft1.insert(self.resource_a, liq_data);
            nft1.insert(self.resource_b, liq_data);
            let mut nft2: HashMap<ResourceAddress, YieldTokenData> = HashMap::new();
            nft2.insert(self.resource_a, yield_token);
            nft2.insert(self.resource_b, yield_token);

            let userdata_multi_nft = self.nft_manager
            .mint_ruid_non_fungible(
                UserMultiPosition {
                    liquidity_position: nft1,
                    yield_token_data: nft2
                }
            );

            scrypto::prelude::NonFungibleBucket(userdata_multi_nft)
        }         



        //unregister from the platform (useful for stokenet test)
        pub fn unregister(&mut self, userdata_nft: Bucket) -> Option<Bucket> {
            //burn the NFT, be sure you'll lose all your tokens not reedemed in advance of this operation
            userdata_nft.burn();
            None
        }

        //supply some tokens
        pub fn extra_supply(&mut self, loan: FungibleBucket, mut data: LiquidityData, token_type: ResourceAddress) -> (Bucket, LiquidityData) {
            //checks amount limits
            let amount_lended = data.amount;
            lend_checks_time_based(amount_lended);
            let num_tokens = loan.amount();
            lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
            info!("Amount of token received: {:?} ", num_tokens);   

            //take the bucket as a new loan and put tokens in corresponding pool
            let vault = self.collected.get_mut(&token_type.clone());
            match vault {
                Some(fung_vault) => {
                    info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                    fung_vault.put(loan);
                }
                None => {
                    assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                }
            }

            //prepare a bucket with TKN tokens to give back to the user 
            let token_received = self.tokenizer_vault.take(num_tokens);
            info!("Returning some fungible token to the account, n° {:?}", token_received.amount());

            // Update the data on the network
            data.start_supply_epoch = Runtime::current_epoch();
            data.end_supply_epoch = Epoch::of(0);
            data.amount = num_tokens;
            // // Insert the modified data back into the hashmap
            // liquidity_position.insert(token_type.clone(), data);
            // self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

            return (token_received, data)    
        }

        //supply some tokens
        pub fn extra(&mut self, loan: FungibleBucket, userdata_nft: NonFungibleBucket, token_type: ResourceAddress, operation: Operation) -> (Bucket, NonFungibleBucket) {
            assert_resource(&userdata_nft.resource_address(), &self.nft_manager.address());
            assert_resource(&loan.resource_address(), &token_type);
            
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut liquidity_position = binding.liquidity_position;

            if let Some(mut data) = liquidity_position.remove(&token_type) {
                info!("Supplying liquidity of type  {:?}, amount {:?}", token_type, data.amount);

                match operation {
                    Operation::Supply => {
                        info!("Storing tokens in the specific vault  {:?}", token_type);
                        let mut modified_data;
                        let mut token_received;
                        (token_received,modified_data) = self.extra_supply(loan,  data,  token_type);

                        // Insert the modified data back into the hashmap
                        liquidity_position.insert(token_type.clone(), data);
                        self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                        return (token_received, userdata_nft)    
                    }
                    Operation::TakesBack => todo!(),
                    Operation::Borrow => todo!(),
                    Operation::Repay => todo!(),
                    Operation::Redeem => todo!(),
                    Operation::Tokenize => todo!(),
                    Operation::Claim => todo!(),
                    Operation::Swap => todo!(), 
                    Operation::No => todo!()
                }
    
            } else {
                //TODO create NFT in case a new token type did not exist at the first time of minting

                let num_tokens = loan.amount();
                lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
                info!("Amount of token received: {:?} ", num_tokens);   

                //take the bucket as a new loan and put tokens in corresponding pool
                let vault = self.collected.get_mut(&token_type.clone());
                match vault {
                    Some(fung_vault) => {
                        info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                        fung_vault.put(loan);
                    }
                    None => {
                        assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                    }
                }

                //prepare a bucket with TKN tokens to give back to the user 
                let token_received = self.tokenizer_vault.take(num_tokens);
                info!("Returning some fungible token to the account, n° {:?}", token_received.amount());

                // Update the data on the network
                let liq_data = self.new_liq_data(Runtime::current_epoch(),Epoch::of(0),num_tokens );
                // Insert the modified data back into the hashmap
                liquidity_position.insert(token_type.clone(), liq_data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                // assert_pair("Unavailable LiquidityPosition data".to_string());
                // let token_received = self.tokenizer_vault.take(dec!(0));
                return (token_received, userdata_nft)                
            }
        }

        //supply some tokens
        pub fn supply(&mut self, loan: FungibleBucket, userdata_nft: NonFungibleBucket, token_type: ResourceAddress) -> (Bucket, NonFungibleBucket) {
            assert_resource(&userdata_nft.resource_address(), &self.nft_manager.address());
            assert_resource(&loan.resource_address(), &token_type);
            
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut liquidity_position = binding.liquidity_position;

            if let Some(mut data) = liquidity_position.remove(&token_type) {
                info!("Supplying liquidity of type  {:?}, amount {:?}", token_type, data.amount);

                //checks amount limits
                let amount_lended = data.amount;
                lend_checks_time_based(amount_lended);
                let num_tokens = loan.amount();
                lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
                info!("Amount of token received: {:?} ", num_tokens);   
    
                //take the bucket as a new loan and put tokens in corresponding pool
                let vault = self.collected.get_mut(&token_type.clone());
                match vault {
                    Some(fung_vault) => {
                        info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                        fung_vault.put(loan);
                    }
                    None => {
                        assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                    }
                }
    
                //prepare a bucket with TKN tokens to give back to the user 
                let token_received = self.tokenizer_vault.take(num_tokens);
                info!("Returning some fungible token to the account, n° {:?}", token_received.amount());
    
                // Update the data on the network
                data.start_supply_epoch = Runtime::current_epoch();
                data.end_supply_epoch = Epoch::of(0);
                data.amount = num_tokens;
                // Insert the modified data back into the hashmap
                liquidity_position.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                return (token_received, userdata_nft)        
            } else {
                //TODO create NFT in case a new token type did not exist at the first time of minting

                let num_tokens = loan.amount();
                lend_amount_checks(num_tokens, self.min_loan_limit, self.max_loan_limit);
                info!("Amount of token received: {:?} ", num_tokens);   

                //take the bucket as a new loan and put tokens in corresponding pool
                let vault = self.collected.get_mut(&token_type.clone());
                match vault {
                    Some(fung_vault) => {
                        info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                        fung_vault.put(loan);
                    }
                    None => {
                        assert_pair("Unavailable resource type for supplying liquidity into".to_string());
                    }
                }

                //prepare a bucket with TKN tokens to give back to the user 
                let token_received = self.tokenizer_vault.take(num_tokens);
                info!("Returning some fungible token to the account, n° {:?}", token_received.amount());

                // Update the data on the network
                let liq_data = self.new_liq_data(Runtime::current_epoch(),Epoch::of(0),num_tokens );
                // Insert the modified data back into the hashmap
                liquidity_position.insert(token_type.clone(), liq_data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                // assert_pair("Unavailable LiquidityPosition data".to_string());
                // let token_received = self.tokenizer_vault.take(dec!(0));
                return (token_received, userdata_nft)                
            }
        }

        //gives back the original token supplied 
        pub fn takes_back(&mut self, refund: Bucket, userdata_nft: NonFungibleBucket, token_type: ResourceAddress) -> (Bucket, Option<NonFungibleBucket>) {
            assert_resource(&userdata_nft.resource_address(), &self.nft_manager.address());

            let non_fung_bucket = userdata_nft.as_non_fungible();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut liquidity_position = binding.liquidity_position;

            if let Some(mut data) = liquidity_position.remove(&token_type) {
                info!("Returning liquidity data of type  {:?}, amount {:?}", token_type, data.amount);

                // Verify the user has requested back at least 20% of its current loan
                take_back_checks(data.amount / 5, &refund.amount());

                // Update the amount field
                let remaining_amount_to_return = data.amount - refund.amount(); 
                info!("Remaining tokens to return: {:?} ", remaining_amount_to_return);   

                //take the bucket to close the loan, and returns corresponding tokens from the main pool
                let amount_to_be_returned = refund.amount();
                self.tokenizer_vault.put(refund);

                //calculate interest over the epochs
                let interest_totals = calculate_interests(
                    &self.reward_type, &self.reward,
                    data.start_supply_epoch.number(),
                    &amount_to_be_returned, &self.interest_for_suppliers);
                info!("Calculated interest {} ", interest_totals);

                //total amount to return 
                let amount_returned = amount_to_be_returned + interest_totals;
                info!("tokens to be given back: {:?} ", amount_returned);  
                
                //total net amount to return
                let vault = self.collected.get_mut(&token_type.clone());
                
                match vault {
                    Some(fung_vault) => {
                        info!("Returning tokens to the account, n°  {:?}", amount_returned);
                        //getting tokens to be returned
                        let bucket_returned = fung_vault.take(amount_returned);

                        // Update the data on the network
                        let nft_local_id: NonFungibleLocalId = userdata_nft.as_non_fungible().non_fungible_local_id();
                        if remaining_amount_to_return == dec!("0") {
                            info!("set the supply operation as 'closed'");
                            //here, we set the supply operation as 'closed' by setting 'end_supply_epoch'
                            data.end_supply_epoch = Runtime::current_epoch();
                            data.amount = remaining_amount_to_return;
                            // Insert the modified data back into the hashmap
                            liquidity_position.insert(token_type.clone(), data);
                            self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                            return (bucket_returned.into(),Some(userdata_nft))              
                        } else {
                            info!("set the supply operation as 'not closed', remaining {}", remaining_amount_to_return);
                            //here, the supply operation is not 'closed' because some tokens are supplied in yet 
                            data.amount = remaining_amount_to_return;
                            // Insert the modified data back into the hashmap
                            liquidity_position.insert(token_type.clone(), data);
                            self.nft_manager.update_non_fungible_data(&nft_local_id, "liquidity_position", liquidity_position);

                            return (bucket_returned.into(),Some(userdata_nft))                
                        }
                    }
                    None => {
                        assert_pair("Unavailable Vault".to_string());
                        let token_received = self.tokenizer_vault.take(dec!(0));
                        return (token_received, None)                            
                    }
                }
            } else {
                assert_pair("Unavailable liquidity_position of the specified token".to_string());
                let token_received = self.tokenizer_vault.take(dec!(0));
                return (token_received, None)      
            }

        }

        // tokenize
        pub fn tokenize_yield(
            &mut self, 
            tkn_token: Bucket,
            tokenize_expected_length: Decimal,
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (FungibleBucket, NonFungibleBucket) {
            // assert_ne!(self.check_maturity(), true, "The expiry date has passed!");
            assert_eq!(tkn_token.resource_address(), self.tokenizer_manager.address());

            epoch_max_length_checks(self.tokenize_epoch_max_lenght,tokenize_expected_length);
            epoch_min(tokenize_expected_length);

            let zsu_amount = tkn_token.amount();
                
            //when you tokenize you fix the interest until the maturity date
            let extra_interest = self.extra_reward;
            info!("Tokenize for n° {} epoch with extra reward {} ", tokenize_expected_length, self.extra_reward); 
                    
            //mint some principal token
            let pt_bucket = self.pt_resource_manager.mint(zsu_amount).as_fungible();
            let maturity_epoch = Decimal::from(Runtime::current_epoch().number()) + tokenize_expected_length;
            //calculate interest
            let accumulated_interest = calculate_interest(tokenize_expected_length, extra_interest, zsu_amount);  
            info!("Simple Interest to be paied {} at epoch {} for the tokenized", accumulated_interest, maturity_epoch);
            
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                info!("Tokenize tokens of type  {:?}, amount {:?}", token_type, zsu_amount);
                info!("Principal returned = {:?}", data.principal_returned);
                if data.principal_returned==true {
                    //lock the tokens
                    self.tokenizer_vault.put(tkn_token);

                    //updates data on NFT       
                    data.extra_reward = self.extra_reward;
                    data.underlying_amount = zsu_amount;
                    data.interest_totals = accumulated_interest;
                    data.yield_claimed = Decimal::ZERO;
                    data.maturity_date = maturity_epoch;
                    data.principal_returned = false;
                    // Insert the modified data back into the hashmap
                    yield_data.insert(token_type.clone(), data);
                    self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                    return (pt_bucket, userdata_nft)
                } else {
                    assert_pair("You already have some tokenized liquidity ".to_string());
                    return (scrypto::prelude::FungibleBucket(tkn_token), userdata_nft)
                }
            } else {
                info!("No Yield Data available");
                //lock the tokens
                self.tokenizer_vault.put(tkn_token);                
                //updates data on NFT
                let strip = YieldTokenData {
                    extra_reward: self.extra_reward,
                    underlying_amount: zsu_amount,
                    interest_totals: accumulated_interest,
                    yield_claimed: Decimal::ZERO,
                    maturity_date: maturity_epoch,
                    principal_returned: false,
                };
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), strip);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);
                info!("New Yield Data has been creted in the account NFT");
                return (pt_bucket, userdata_nft)
            }
        }     

        //swap  todo -> to be implemented
        pub fn redeem(
            &mut self, 
            pt_bucket: Bucket, 
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, Option<NonFungibleBucket>) {
            
            let pt_redeem_amount = pt_bucket.amount();
            assert_eq!(pt_bucket.resource_address(), self.pt_resource_manager.address());
            assert_eq!(userdata_nft.resource_address(), self.nft_manager.address());

            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(data) = yield_data.remove(&token_type) {
                info!("Swap tokens, current_epoch = {:?}, amount = {:?} , interest {:?}, maturity date {:?}, type  {:?}, ", 
                        Runtime::current_epoch().number(), pt_redeem_amount, data.interest_totals, data.maturity_date, token_type);

                assert_eq!(pt_redeem_amount, data.underlying_amount,
                    "You need to swap all your tokenized value!");
                // total at maturity * (1/(1+0.08)) -> 10,240 * 0.9259 -> 9,481
                // maucalay duration = 9,481/10,000 -> 0.948
                // modified duration = maucalay duration / (1+0.08) -> 0,948 / 1,08 = 0,877
                let fixed_reward = data.extra_reward/dec!(100);
                let total_at_maturity =  (data.underlying_amount+data.interest_totals) * (1/(1+fixed_reward));         //data.underlying_amount          
                info!("total at maturity {:?}", total_at_maturity);
                let maucalay_duration = total_at_maturity/data.underlying_amount; //data.underlying_amount
                info!("maucalay duration {:?}", maucalay_duration);
                let modified_duration = maucalay_duration / (1+fixed_reward);
                info!("modified duration {:?}", modified_duration);

                //differences in % from the time when the tokenize occurred and now
                //positive value means that % has been lowered -> PT value has risen
                //negative value means that % has been raised -> PT value has decreased
                let diff_reward = data.extra_reward - self.extra_reward;
                info!("tokenized reward {}% actual extra_reward {}% diff_reward {:?}%", data.extra_reward, self.extra_reward, diff_reward);

                // Insert the cleaned data back into the hashmap for the next round of tokenize
                let yield_token = self.init_yield();
                yield_data.insert(token_type.clone(), yield_token);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                //burn principal token because they have been returned as an equivalent 
                pt_bucket.burn();
                //return back the amount priced at the current value
                let diff = diff_reward*modified_duration;
                info!("returned value is higher/lower of about {:?} %", diff);
                let priced_amount = (data.underlying_amount+data.interest_totals)*(dec!(100)+diff)/dec!(100);
                //The actualized price of the tokenized supply will be returned
                info!("tokens returned {:?}", priced_amount);
                //unlock the tokens                    
                let zsu_bucket = self.tokenizer_vault.take(priced_amount);

                return (zsu_bucket, Some(userdata_nft))
            } else {
                return (pt_bucket, Some(userdata_nft))
            }
        } 
                       
            

        /// This is for claiming principal token after maturity, you get back the principal that had been tozeniked
        pub fn redeem_from_pt(
            &mut self,
            pt_bucket: FungibleBucket,
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, NonFungibleBucket) {

            info!("Returning PT amount {}", pt_bucket.amount());   
            assert_eq!(pt_bucket.resource_address(), self.pt_resource_manager.address());

            //update principal returned
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                // To redeem PT only, must wait until after maturity.
                assert_eq!(
                    check_maturity(data.maturity_date), 
                    false, 
                    "The Principal token has NOT reached its maturity!"
                );
                let zsu_amount = pt_bucket.amount();    
                info!("Reedem PT amount {:?} of type  {:?}", zsu_amount, token_type);

                //set that the principal has been returned
                data.principal_returned = true;
                data.extra_reward = dec!(0);
                data.underlying_amount = dec!(0);
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                //return the amount that was tokenized        
                let bucket_of_zsu = self.tokenizer_vault.take(zsu_amount);
                pt_bucket.burn();
   
                return (bucket_of_zsu, userdata_nft)
            } else {
                assert_pair("No PT available".to_string());
                return (pt_bucket.into(), userdata_nft)
            }
        }

        /// This is for claiming yield after maturity, you get back the interest calculated at the time of tozeniking
        pub fn claim_yield(
            &mut self, 
            // _yt_proof: NonFungibleProof,
            userdata_nft: NonFungibleBucket,
            token_type: ResourceAddress
        ) -> (Bucket, NonFungibleBucket) {

            //update principal returned
            let non_fung_bucket = userdata_nft.as_non_fungible();
            let nft_local_id: NonFungibleLocalId = non_fung_bucket.non_fungible_local_id();
            let binding = non_fung_bucket.non_fungible::<UserMultiPosition>().data();
            let mut yield_data = binding.yield_token_data;

            if let Some(mut data) = yield_data.remove(&token_type) {
                // To claim yield only, must wait until after maturity.
                assert_eq!(
                    check_maturity(data.maturity_date), 
                    false, 
                    "The yield token has NOT reached its maturity!"
                );
                
                let interest_totals = data.interest_totals;
                info!("Paying back interest {} ", interest_totals);   
                //total net amount to return
                let net_returned = self.tokenizer_vault.take(interest_totals);
    
                //update claimed yield
                data.interest_totals = dec!(0);
                data.yield_claimed = interest_totals;
                // Insert the modified data back into the hashmap
                yield_data.insert(token_type.clone(), data);
                self.nft_manager.update_non_fungible_data(&nft_local_id, "yield_token_data", yield_data);

                (net_returned, userdata_nft)
            } else {
                assert_pair("No Yield available".to_string());
                let net_returned = self.tokenizer_vault.take(dec!(0));
                return (net_returned, userdata_nft)
            }
        }

        //Utility functions
        // set the reward for suppliers
        pub fn set_reward(&mut self, reward: Decimal) {
            self.reward = reward;
            self.interest_for_suppliers.insert(Decimal::from(Runtime::current_epoch().number()), reward);
        }

            pub fn set_extra_reward(&mut self, extra_reward: Decimal) {
                self.extra_reward = extra_reward;
            }

        //set the reward type, if fixed or timebased
        pub fn set_reward_type(&mut self, reward_type: String) {
            self.reward_type = reward_type
        }

        //extend the pool for accept lendings
        pub fn extend_lending_pool(&mut self, size_extended: Decimal) {
            self.tokenizer_vault.put(self.tokenizer_manager.mint(size_extended));
        }

        //for funding the main pool
        pub fn fund_main_pool(&mut self, fund: FungibleBucket)  {
            info!("Fund received to fund the main vault: {:?} ", fund.amount());  
            let token_type = fund.resource_address();

            //take the  bucket as a new loan and put tokens in corresponding pool
            let vault = self.collected.get_mut(&token_type.clone());
            match vault {
                Some(fung_vault) => {
                    info!("Receiving a supply of {:?}, amount {:?}", token_type, fund.amount());
                    info!("Storing tokens in the specific vault  {:?}", fung_vault.resource_address());
                    fung_vault.put(fund);
                }
                None => {
                    info!("Unavailable pair ");
                }
            }
        }


        //for admin only
        pub fn config(&mut self, reward: Decimal, extra_reward: Decimal
                , tokenize_epoch_max_lenght: Decimal
                , min_loan_limit: Decimal, max_loan_limit: Decimal ) {                
            self.set_reward(reward);
    
            //without methods
            self.tokenize_epoch_max_lenght = tokenize_epoch_max_lenght;
            self.extra_reward = extra_reward;
            self.min_loan_limit = min_loan_limit; //min limit 
            self.max_loan_limit = max_loan_limit; //max limit 
       
            assert!(
                max_loan_limit > min_loan_limit,
                "Maximum number of tokens must be higher than Min limit"
            );                                           
        }      

        fn init_yield(&mut self) -> YieldTokenData {
            return YieldTokenData {
                extra_reward: dec!(0),
                underlying_amount: dec!(0),
                interest_totals: dec!(0),
                yield_claimed: dec!(0),
                maturity_date: dec!(0),
                principal_returned: true,
            };    
        }

        fn init_liq_data(&mut self) -> LiquidityData {
            return LiquidityData {
                start_supply_epoch: Epoch::of(0),
                end_supply_epoch: Epoch::of(0),
                amount: dec!("0"),
            };
        }   

        fn new_liq_data(&mut self,start_supply_epoch: Epoch,end_supply_epoch: Epoch,amount: Decimal) -> LiquidityData {
            return LiquidityData {
                start_supply_epoch: start_supply_epoch,
                end_supply_epoch: end_supply_epoch,
                amount: amount
            };
        }         

        //mint a staff for a new staff member
        pub fn mint_staff_badge(&mut self, username: String) -> Bucket {
            let staff_badge_bucket: Bucket = self
                .staff_badge_resource_manager
                .mint_ruid_non_fungible(StaffBadge {
                    username: username.clone(),
                });

            let id = staff_badge_bucket.as_non_fungible().non_fungible_local_id();
            let key = self.staff.get_length().to_u16().unwrap()+1; 
            info!("Saving staff badge with key : {:?} and id {:?} for the username: {:?}  ",key, id, username);
            self.staff.insert(key, id);

            staff_badge_bucket
        }
    
    }
}