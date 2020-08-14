
// The token debit function
function debit(
    input,
    token_debit: u8[48],
    token_credit: u8[48]
) -> (u8, u8) {
    // 1. Check token ids match
    let id_t = input.registers.id;
    let payload_id_t = input.record.payload[0]; // payload is u8 bytes
    assert_eq!(id_t, payload_id_t);

    // 2. Check record death is token_debit
    let id_d = input.record.death_program_id;
    assert_eq!(id_d, token_debit);

    // 3. Check record birth is token_credit
    let id_b = input.record.birth_program_id;
    assert_eq!(id_b, token_credit);

    // 4. Handle old_record_0 new token case
    let global_index = input.state.leaf_index;
    let vb_t_old = input.registers.value_balance;
    if global_index == 0 {
        assert_eq!(vb_t_old, 0);
    }

    // 5. Add payload value to register value balance
    let payload_vb_t = input.record.payload[1];
    let vb_t_new = vb_t_old + payload_vb_t;

    // 6. Check that no money is lost
    let no_lost_money = vb_t_new >= vb_t_old;
    assert_eq!(no_lost_money, true);

    print!("balance before {}", vb_t_old);
    print!("balance after {}", vb_t_new);
    print!("no lost money {}", no_lost_money);

    return (id_t, vb_t_new)
}

// The token credit function
function credit(
    input,
    token_credit: u8[48]
) -> (u8, u8) {
    let id_t = input.registers.id;
    let payload_id_t = input.record.payload[0];

    // 1. Check token id's match
    assert_eq!(id_t, payload_id_t);

    // 2. Check record birth
    let id_b = input.record.birth_program_id;
    assert_eq!(id_b, token_credit);

    // 3. Subtract payload value form register value balance
    let vb_t_old = input.registers.value_balance;
    let payload_vb_t = input.record.payload[1];
    let vb_t_new = vb_t_old - payload_vb_t;

    // 4. Check that no money is created
    let no_new_money = vb_t_new <= vb_t_old;
    assert_eq!(no_new_money, true);

    // 5. Handle new_record_1 value is conserved case
    let global_index = input.state.leaf_index;
    if global_index == 3 {
        assert_eq!(vb_t_new, 0);
    }

    print!("old value {}", vb_t_old);
    print!("new value {}", vb_t_new);
    print!("no new money {}", no_new_money);

    return (id_t, vb_t_new)
}

// The token main function
function main(input, token_debit: u8[48], token_credit: u8[48]) -> (u8, u8) {

    let (id_t, vb_t) = debit(input, token_debit, token_credit);

    //let (id_t, vb_t) = credit(input, token_credit);

    return (id_t, vb_t)
}