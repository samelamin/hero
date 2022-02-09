const Sample = artifacts.require("Incrementor");

module.exports = function (deployer) {
  deployer.deploy(Sample);
};