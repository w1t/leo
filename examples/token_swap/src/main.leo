import debit.exchange_token_debit;
import swap.exchange_or_cancel;
import credit.(exchange_token_0_credit, exchange_token_1_credit);

// The 'token_swap' main function.
function main(
    input,
    is_exchange: bool,
    refund: address
) -> (u8, u8, address, address, u8, u8) {

    // State 0
    //let (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1) = exchange_token_debit(input);

    // State 1
    //let (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1) = exchange_or_cancel(input, is_exchange, refund);

    // State 2
    let (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1) = exchange_token_0_credit(input);

    // State 3
    //let (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1) = exchange_token_1_credit(input);

    return (id_0, id_1, maker_recv, taker_recv, vb_0, vb_1)
}
