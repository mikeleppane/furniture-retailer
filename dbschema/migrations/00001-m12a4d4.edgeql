CREATE MIGRATION m12a4d4bgnujmunmxhq4xbv2ru74socdr6g53vjaudx5y6y7a5mw3q
    ONTO initial
{
  CREATE TYPE default::Batch {
      CREATE MULTI PROPERTY allocations: array<std::json>;
      CREATE PROPERTY eta: std::str;
      CREATE REQUIRED PROPERTY purchased_quantity: std::int32;
      CREATE REQUIRED PROPERTY ref_: std::str;
      CREATE REQUIRED PROPERTY sku: std::str;
  };
  CREATE TYPE default::OrderLine {
      CREATE PROPERTY orderid: std::str;
      CREATE PROPERTY qty: std::int32;
      CREATE PROPERTY sku: std::str;
  };
};
