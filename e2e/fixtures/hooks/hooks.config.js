
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

module.exports = {
  async buildStart() {
    console.log('>> build start');
    await delay(1000);
    console.log('>> build start after delay 1s');
  },
  async generateEnd() {
    console.log('>> generate end');
    await delay(1000);
    console.log('>> generate end after delay 1s');
  },
  async load(path) {
    console.log('>> load:', path);
  }
};
