CREATE MIGRATION m1eno76grzin2bqy7vbgelj4hf7jjhtmpjfxuf5t32zkarqgqpq44q
    ONTO m136v3kjub2z7lmzginso44zlgpd66tdljtkf44mggo7isrkl2dljq
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY allocations {
          RESET CARDINALITY USING (SELECT
              .allocations 
          LIMIT
              1
          );
      };
  };
};
