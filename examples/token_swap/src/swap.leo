// The `exchange_or_cancel` function
function exchange_or_cancel(
    input,
    is_exchange: bool,
    refund: address
) -> (u8, u8, address, address, u8, u8) {
    // NOT SAFE - set ids to constant bytes
    const exchange_or_cancel: u8[48] = [3u8; 48];

    //1. Check record death is exchange_token_debit
    let record_death = input.record.death_program_id;
    assert_eq!(exchange_or_cancel, record_death);

    if is_exchange {
        // 2. Check the record is old record 1
        let record_index = input.state.leaf_index;
        assert_eq!(1, record_index);

        // 3. Check registers maker_recv matches
        let maker_recv = input.registers.maker_recv;
        let maker = input.record.owner;
        // assert_eq!(maker_recv, maker); //TODO: address comparison

        // 4. Check registers token ids match
        let id_1 = input.registers.id_1;
        let id_0 = input.registers.id_0;
        let payload_id_0 = input.record.payload[0];
        let payload_id_1 = input.record.payload[2];
        assert_eq!(id_0, payload_id_0);
        assert_eq!(id_1, payload_id_1);

        //5. Check registers value balances match
        let mut vb_0 = input.registers.vb_0;
        let vb_1 = input.registers.vb_1;
        let expected_vb_1 = input.record.payload[3];
        assert_eq!(vb_0, 0);
        assert_eq!(vb_1, expected_vb_1);

        //6. Add payload value to register vb_0
        let payload_vb_0 = input.record.payload[1];
        vb_0 += payload_vb_0;

        //7. Set register taker_recv to receiver address
        let taker_recv = input.registers.taker_recv;

        return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
    } else {
        //2. Check refund address matches
        let maker_recv = input.record.owner;
        //assert_eq!(refund, maker_recv); // TODO: impl address equality

        //3. Check registers token ids match
        let id_0 = input.registers.id_0;
        let id_1 = input.registers.id_1;
        let payload_id_0 = input.record.payload[0];
        let payload_id_1 = input.record.payload[2];
        assert_eq!(id_0, payload_id_0);
        assert_eq!(id_1, payload_id_1);

        //4. Check registers values are 0
        let mut vb_0 = input.registers.vb_0;
        let vb_1 = input.registers.vb_1;
        assert_eq!(vb_0, 0);
        assert_eq!(vb_1, 0);

        //5. Add payload value to register vb_0
        let payload_vb_0 = input.record.payload[1];
        vb_0 += payload_vb_0;

        //6. Set taker_recv to the refund address
        let taker_recv = maker_recv;

        return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
    }
}