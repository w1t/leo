
// The stable token debit function
function debit(
    input,
    token_debit: u8[48],
    token_credit: u8[48],
    central_authority: address
) -> (u8, u8) {
    // 1. Check token ids match
    let id_t = input.registers.id;
    let payload_id_t = input.record.payload[0];
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

    // 5. Check that policy signature is valid
    // TODO: enforce that policy is signed by central authority
    // let verified = signature.verify(policy, central_authority);
    // assert_eq!(verified, true);

    //let policy = input.record.payload[2];
    //let fake_check = policy != 0;
    //assert_eq!(fake_check, true);

    //let signature = input.record.payload[3];
    //let fake_check = signature != 0;
    //assert_eq!(fake_check, true);

    // 6. Enforce policy
    // TODO: check that owner is not a forbidden address
    // let blacklisted = policy.blacklist.contains(owner);
    // assert_eq!(blacklisted, false);
    // let owner = input.record.owner;

    // 7. Add payload value to register value balance
    let payload_vb_t = input.record.payload[1];
    let vb_t_new = vb_t_old + payload_vb_t;

    // 8. Check that no money is lost
    let no_lost_money = vb_t_new >= vb_t_old;
    assert_eq!(no_lost_money, true);

    return (id_t, vb_t_new)
}

// The stable token credit function
function credit(
    input,
    token_credit: u8[48],
    central_authority: address
) -> (u8, u8) {
    let id_t = input.registers.id;
    let payload_id_t = input.record.payload[0];

    // 1. Check token id's match
    assert_eq!(id_t, payload_id_t);

    // 2. Check record birth
    let id_b = input.record.birth_program_id;
    assert_eq!(id_b, token_credit);

    // 3. Check that policy signature is valid
    // TODO: enforce that policy is signed by central authority
    // let verified = signature.verify(policy, central_authority);
    // assert_eq!(verified, true);

    //let policy = input.record.payload[2];
    //let fake_check = policy != 0;
    //assert_eq!(fake_check, true);

    //let signature = input.record.payload[3];
    //let fake_check = signature != 0;
    //assert_eq!(fake_check, true);

    // 4. Enforce policy
    // TODO: check that owner is not a forbidden address
    // let blacklisted = policy.blacklist.contains(owner);
    // assert_eq!(blacklisted, false);
    // let owner = input.record.owner;

    // 5. Subtract payload value form register value balance
    let vb_t_old = input.registers.value_balance;
    let payload_vb_t = input.record.payload[1];
    let vb_t_new = vb_t_old - payload_vb_t;

    // 6. Check that no money is created
    let no_new_money = vb_t_new <= vb_t_old;
    assert_eq!(no_new_money, true);

    // 7. Handle new_record_1 value is conserved case
    let global_index = input.state.leaf_index;
    if global_index == 3 {
        assert_eq!(vb_t_new, 0);
    }

    return (id_t, vb_t_new)
}

// The stable token main function
function main(
    input,
    token_debit: u8[48],
     token_credit: u8[48],
     central_authority: address
 ) -> (u8, u8) {

    let (id_t, vb_t) = debit(input, token_debit, token_credit, central_authority);

    //let (id_t, vb_t) = credit(input, token_credit, central_authority);

    return (id_t, vb_t)
}