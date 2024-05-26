module default {
    type OrderLine {
        required orderid: str {
            constraint min_len_value(1);
            constraint max_len_value(255);
            constraint exclusive;
        }
        required qty: int32 {
            constraint min_value(0);
        }
        required sku: str {
            constraint min_len_value(1);
            constraint max_len_value(255);
        }
    }
    type Batch {
        required reference: str {
            constraint min_len_value(1);
            constraint max_len_value(255);
            constraint exclusive;
        }
        required purchased_quantity: int32 {
            constraint min_value(0);
        }
        required sku: str {
            constraint min_len_value(1);
            constraint max_len_value(255);
        }
        eta: cal::local_date;
        multi allocations: OrderLine;
    }
}
