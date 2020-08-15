// The `exchange_token_debit` function
function exchange_token_debit(input) -> (u8, u8, address, address, u8, u8) {
    // NOT SAFE - set ids to constant bytes
    const exchange_token_debit: u8[48] = [1u8; 48];
    const exchange_token_credit: u8[48] = [3u8; 48];

    // 1. Check the record is old record 0
    let record_index = input.state.leaf_index;
    assert_eq!(0, record_index);

    // 2. Check registers token ids match
    let id_1 = input.registers.id_1;
    let payload_id_1 = input.record.payload[0];
    assert_eq!(id_1, payload_id_1);

    //3. Check record death is exchange_token_debit
    let record_death = input.record.death_program_id;
    assert_eq!(exchange_token_debit, record_death);

    //4. Check record birth is exchange_token_credit
    let record_birth = input.record.birth_program_id;
    assert_eq!(exchange_token_credit, record_birth);

    //5. Check registers value balances are 0
    let vb_0 = input.registers.vb_0;
    let mut vb_1 = input.registers.vb_1;
    assert_eq!(vb_0, 0);
    assert_eq!(vb_1, 0);

    //6. Add payload value to register vb_1
    let payload_vb = input.record.payload[1];
    vb_1 += payload_vb;

    //7. Set register taker_recv to receiver address
    let taker_recv = input.record.owner;

    let maker_recv = input.registers.maker_recv;
    let id_0 = input.registers.id_0;

    return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
}