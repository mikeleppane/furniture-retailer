CREATE MIGRATION m17xaambfb5dtmxpjzfwo2efxjsqf2zz5ttuh4f6jiafjunynb4rka
    ONTO m1eno76grzin2bqy7vbgelj4hf7jjhtmpjfxuf5t32zkarqgqpq44q
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY eta {
          SET TYPE std::datetime USING (<std::datetime>.eta);
      };
  };
};
