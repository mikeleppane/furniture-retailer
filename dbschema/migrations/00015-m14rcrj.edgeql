CREATE MIGRATION m14rcrj5uhmj4dyorxscjmlj7ays4jhxvfojp57jvc6bed7hy3kw4a
    ONTO m1q3n2nmrmpjtoohhqtiinjs4nyfnfsbsewd7jyklul23a4jovc2ga
{
  ALTER TYPE default::Batch {
      CREATE MULTI LINK allocations: default::OrderLine;
  };
};
