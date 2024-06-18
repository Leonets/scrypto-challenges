use resource::{ScryptoBucket, ScryptoNonFungibleBucket};
use tokenizer::tokenizer::{test_bindings::*, UserMultiPosition};
use scrypto::*;
use scrypto_test::prelude::*;
use scrypto::prelude::FungibleBucket;


/// 
/// A test to verify supply, tokenize with multiple accounts
/// 
#[test]
fn tokenizer_supply_tokenize_multiple() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_tokenize_swap_success_test: {:?} ", package_address); 
    
    // Create a bucket of XRD
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    
    // Create another
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token2 = ScryptoBucket::resource_address(&bucket2);
        

    // Create a bucket of XRD
    // Create a bucket of XRD and Token2
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;
    let initial_fund2 = BucketFactory::create_fungible_bucket(token2,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";
    // Instantiate the component
    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token2, package_address, &mut env,)?;

    // Fund main pool
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund2), env);
        /* Kernel modules are reset just after this point. */
    });
    // //Create another account
    // let account = env
    // .call_function_typed::<_, AccountCreateAdvancedOutput>(
    //     ACCOUNT_PACKAGE,
    //     ACCOUNT_BLUEPRINT,
    //     ACCOUNT_CREATE_ADVANCED_IDENT,
    //     &AccountCreateAdvancedInput {
    //         owner_role: OwnerRole::None,
    //         address_reservation: None,
    //     },
    // )
    // .unwrap()
    // .0;

    // Register an account
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    assert_eq!(userdata_nft.amount(), dec!("1"));
    println!("Nft: {:?} ", userdata_nft);  

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Supply
    let liquid_bucket = tokenizerdapp.supply(bucket1, userdata_nft_proof.into(), XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.as_fungible().amount();
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));
    
    // Tokenize
    let tokenized_bucket = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    assert_eq!(tokenized_bucket.as_fungible().amount(), amount);


    env.set_current_epoch(Epoch::of(10100));
    // Act
    let liquid_bucket = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    assert_eq!(liquid_bucket.as_fungible().amount(), amount);
    println!("liquid_bucket from reedem: {:?} ", liquid_bucket.as_fungible().amount());  

    // Act
    let liquid_bucket = tokenizerdapp.claim_yield(userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    println!("liquid_bucket from claim: {:?} ", liquid_bucket.as_fungible().amount());  
    assert_eq!(liquid_bucket.as_fungible().amount(), dec!(0.95583));
    

    //supply and tokenize with the second token 
    // Supply
    let loan = scrypto::prelude::FungibleBucket(bucket2);
    println!("Ready to supply again ");  
    let liquid_bucket2 = tokenizerdapp.supply(loan, userdata_nft.create_proof_of_all().into(), token2, &mut env)?;
    
    // Tokenize
    println!("Ready to tokenize again ");  
    let _tokenized_bucket2 = tokenizerdapp.tokenize_yield(liquid_bucket2, dec!(10000), userdata_nft.create_proof_of_all().into(), token2,&mut env)?;
    
    println!("Ready to display Nft ");  
    // let nft_data: UserMultiPosition = userdata_nft.as_non_fungible().into();   

    let non_fung_bucket = resource::ScryptoBucket::as_non_fungible(&userdata_nft);
    println!("Quite ready to display Nft ");  
    let nft_data = resource::ScryptoNonFungibleBucket::non_fungible::<UserMultiPosition>(&non_fung_bucket).data();
    println!("Finally to display Nft ");  

    // let yield_data: HashMap<ResourceAddress, YieldTokenData> = nft_data.yield_token_data;
    // let liquidity_data: HashMap<ResourceAddress, LiquidityData> = nft_data.liquidity_position;

    // println!("Why not ?? ");  
    // for (k, v) in liquidity_data {
    //     println!("resourceAddress {:?}: Liquidity Data {:?}", k, v);
    // }

    // for (k, v) in yield_data {
    //     println!("resourceAddress {:?}: Yield Data {:?}", k, v);
    // }
    nft_data.log_contents();
    // let mut yield_data = nft_data.yield_token_data;
    // log_user_position(&nft_data);

    Ok(())
}    


/// 
/// A test to verify supply, tokenize and swap
/// 
#[test]
fn tokenizer_supply_tokenize_swap_success_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_tokenize_swap_success_test: {:?} ", package_address); 
    
    // Create a bucket of XRD
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    
    // Create another
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = ScryptoBucket::resource_address(&bucket2);

    // Create a bucket of XRD
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";
    // Instantiate the component
    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Fund main pool
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Register an account
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    println!("Nft: {:?} ", userdata_nft);  

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Supply
    let liquid_bucket = tokenizerdapp.supply(bucket1, userdata_nft_proof.into(), XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.as_fungible().amount();
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));
    
    // Tokenize
    let tokenized_bucket = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    assert_eq!(tokenized_bucket.as_fungible().amount(), amount);

    env.set_current_epoch(Epoch::of(1100));
    
    // Swap before maturity
    let liquid_bucket = tokenizerdapp.trade(tokenized_bucket.into(), userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    assert_eq!(liquid_bucket.as_fungible().amount(), dec!(100.95583));
    println!("liquid_bucket from swap: {:?} ", liquid_bucket.as_fungible().amount());  

    Ok(())
}


/// 
/// A test to verify supply 
/// 
#[test]
fn tokenizer_supply_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    // Create a bucket of XRD
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    // Create a bucket 
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = ScryptoBucket::resource_address(&bucket2);

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";
    // Instantiate the component
    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Register an account
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    info!("Nft: {:?} ", _nft_bucket);  

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Supply
    let liquid_bucket = tokenizerdapp.supply(bucket1, userdata_nft_proof.into(), XRD, &mut env)?;

    // Assert
    assert_eq!(liquid_bucket.as_fungible().amount(), dec!("100"));
    Ok(())
}


/// 
/// A test to verify supply and withdraw
/// 
#[test]
fn tokenizer_takes_back_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();

    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    // Create a bucket of XRD
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    // Create a bucket
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = ScryptoBucket::resource_address(&bucket2);

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";
    // Instantiate the component
    let (mut tokenizerdapp, _admin_badge, _owner_badge) = Tokenizer::instantiate(
        reward, symbol, reward_type.to_owned(), XRD, token1, package_address, &mut env,)?;
    
    // Fund main pool
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });
    // Register an account
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Supply
    let liquid_bucket = tokenizerdapp.supply(bucket1,  userdata_nft_proof.into() , XRD, &mut env)?;

    // Verify that the received buckets amount matches the expected amount
    // Assert
    assert_eq!(liquid_bucket.as_fungible().amount(), dec!("100"));

    info!("Nft: {:?} ", _nft_bucket);  

    env.set_current_epoch(Epoch::of(10000));
    // Withdraw
    let xrd_bucket= tokenizerdapp.takes_back(liquid_bucket, userdata_nft.create_proof_of_all().into(),XRD, &mut env)?;

    assert_eq!(xrd_bucket.as_fungible().amount(), dec!("100.47668"));

    Ok(())
}


/// 
/// A test to verify supply , tokenize and redeem
/// 
#[test]
fn tokenizer_supply_and_tokenize_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_and_tokenize_test: {:?} ", package_address); 
    
    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = ScryptoBucket::resource_address(&bucket2);

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Register an account
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    println!("Nft: {:?} ", userdata_nft);  

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Supply
    let liquid_bucket = tokenizerdapp.supply(bucket1, userdata_nft_proof.into(), XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.as_fungible().amount();
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));
    // Tokenize
    let tokenized_bucket = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), userdata_nft.create_proof_of_all().into(),XRD, &mut env)?;
    assert_eq!(tokenized_bucket.as_fungible().amount(), amount);

    env.set_current_epoch(Epoch::of(9999));
    // Reedem
    let _result = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft.create_proof_of_all().into(), XRD,&mut env);

    // Assert that the function panicked
    // assert!(result.is_err());
    // if let Err(err) = result {
    //     // Optionally, assert the panic message or type
    //     let panic_message = err.downcast_ref::<&str>().unwrap();
    //     assert_eq!(panic_message, &"Expected error");
    // }

    Ok(())
}



#[test]
fn tokenizer_supply_and_tokenize_success_test() -> Result<(), RuntimeError> {
    // Arrange
    let mut env = TestEnvironment::new();
    let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

    println!("tokenizer_supply_and_tokenize_success_test: {:?} ", package_address); 
    
    // Act
    let bucket1: FungibleBucket = scrypto::prelude::FungibleBucket(BucketFactory::create_fungible_bucket(
        XRD,
        100.into(),
        Mock,
        &mut env
    )?);
    let bucket2 = ResourceBuilder::new_fungible(OwnerRole::None)
    .divisibility(18)
    .mint_initial_supply(100, &mut env)?;
    let token1 = ScryptoBucket::resource_address(&bucket2);

    // Act
    let initial_fund = BucketFactory::create_fungible_bucket(XRD,1000.into(),Mock,&mut env)?;

    let reward = Decimal::from(5);
    let symbol = String::from("TKN");
    let reward_type = "timebased";

    let (mut tokenizerdapp, _admin_badge, _staff_badge) = Tokenizer::instantiate(
        reward,symbol,  reward_type.to_string(), XRD, token1, package_address, &mut env,)?;

    // Act
    let _unused = env.with_auth_module_disabled(|env| {
        /* Auth Module is disabled just before this point */
        let _ = tokenizerdapp.fund_main_pool(scrypto::prelude::FungibleBucket(initial_fund), env);
        /* Kernel modules are reset just after this point. */
    });

    // Act
    let userdata_nft = tokenizerdapp.register(&mut env)?;

    // Verify that the NFT's amount matches the expected amount
    println!("Nft: {:?} ", userdata_nft);   

    let userdata_nft_proof = userdata_nft.create_proof_of_all();

    // Act
    let liquid_bucket = tokenizerdapp.supply(bucket1, userdata_nft_proof, XRD, &mut env)?;

    // Assert
    let amount = liquid_bucket.as_fungible().amount();
    assert_eq!(amount, dec!("100"));

    env.set_current_epoch(Epoch::of(100));

    // Act
    let tokenized_bucket = tokenizerdapp.tokenize_yield(liquid_bucket, dec!(10000), userdata_nft.create_proof_of_all().into(),XRD, &mut env)?;
    assert_eq!(tokenized_bucket.as_fungible().amount(), amount);

    env.set_current_epoch(Epoch::of(11001));
    // Act
    let liquid_bucket = tokenizerdapp.redeem_from_pt(tokenized_bucket, userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    assert_eq!(liquid_bucket.as_fungible().amount(), amount);
    println!("liquid_bucket from reedem: {:?} ", liquid_bucket.as_fungible().amount());  

    // Act
    let liquid_bucket = tokenizerdapp.claim_yield(userdata_nft.create_proof_of_all().into(), XRD,&mut env)?;
    println!("liquid_bucket from claim: {:?} ", liquid_bucket.as_fungible().amount());  
    assert_eq!(liquid_bucket.as_fungible().amount(), dec!(0.95583));

    Ok(())
}


