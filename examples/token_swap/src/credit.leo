// The `exchange_token_credit` function
function exchange_token_0_credit(input) -> (u8, u8, address, address, u8, u8) {
    // NOT SAFE - do not set ids to constant bytes
    const exchange_token_0_credit: u8[48] = [3u8; 48];

    //1. Check record birth is exchange_token_credit
    let record_birth = input.record.birth_program_id;
    assert_eq!(exchange_token_0_credit, record_birth);

    //2. Assert record index is new record 0
    let record_index = input.state.leaf_index;
    assert_eq!(record_index, 2);

    //3. Check the record owner is the taker's recv address
    let taker_recv = input.registers.taker_recv;
    let owner = input.record.owner;
    // assert_eq!(taker_recv, owner); TODO: address equality

    //4. Check registers token id_0s match
    let id_0 = input.registers.id_0;
    let payload_id_0 = input.record.payload[0];
    assert_eq!(id_0, payload_id_0);

    //5. Subtract payload value to register vb_0
    let mut vb_0 = input.registers.vb_0;
    let payload_vb = input.record.payload[1];
    vb_0 -= payload_vb;

    //6. Check register vb_0 is 0
    assert_eq!(vb_0, 0);

    let id_1 = input.registers.id_1;
    let vb_1 = input.registers.vb_1;
    let maker_recv = input.registers.maker_recv;

    return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
}

// The `exchange_token_1_credit` function
function exchange_token_1_credit(input) -> (u8, u8, address, address, u8, u8) {
    // NOT SAFE - do not set ids to constant bytes
    const exchange_token_1_credit: u8[48] = [4u8; 48];

    //1. Check record birth is exchange_token_1_credit
    let record_birth = input.record.birth_program_id;
    assert_eq!(exchange_token_1_credit, record_birth);

    //2. Assert record index is new record 1
    let record_index = input.state.leaf_index;
    assert_eq!(record_index, 3);

    //3. Check the record owner is the maker's recv address
    let maker_recv = input.registers.maker_recv;
    let owner = input.record.owner;
    // assert_eq!(maker_recv, owner); TODO: address equality

    //4. Check registers token id_1s match
    let id_1 = input.registers.id_1;
    let payload_id_1 = input.record.payload[0];
    assert_eq!(id_1, payload_id_1);

    //5. Subtract payload value to register vb_1
    let mut vb_1 = input.registers.vb_1;
    let payload_vb = input.record.payload[1];
    vb_1 -= payload_vb;

    //6. Check registers are 0
    let vb_0 = input.registers.vb_0;
    assert_eq!(vb_0, 0);
    assert_eq!(vb_1, 0);

    let id_0 = input.registers.id_0;
    let taker_recv = input.registers.taker_recv;

    return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
}

