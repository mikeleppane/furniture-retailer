CREATE MIGRATION m1vravofqa3wkiaptkqvs7eeoyg275hk3qx4db77csi7jr5lttyltq
    ONTO m17xaambfb5dtmxpjzfwo2efxjsqf2zz5ttuh4f6jiafjunynb4rka
{
  ALTER TYPE default::Batch {
      ALTER PROPERTY eta {
          SET TYPE std::str USING (<std::str>.eta);
      };
  };
};
