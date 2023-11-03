import { Registry, set_logger } from './pkg';

set_logger();
let REGISTRY;

const button = document.getElementById('setAttributeButton'); 
const spinner = document.getElementById('loading');
const transactionReceipt = document.getElementById('transactionReceipt');
const spinnerText = document.getElementById('spinnerText');
spinner.style.display = 'block';
spinnerText.textContent = "Deploying Contract...";
init_registry();


button.addEventListener('click', async () => {
  await setAttributeWithValue();
});

async function setAttributeWithValue() {
  if (!REGISTRY) {
    console.error("Registry not initialized");
    return;
  }

  const attributeInput = document.getElementById('attributeInput');
  try {
    const inputValue = attributeInput ? attributeInput.value : '';
    console.log("Setting attribute ", inputValue);
    spinner.style.display = 'block';
    spinnerText.textContent = "Submitting Transaction";
    let output = await REGISTRY.set_attribute(inputValue.toString());
    spinner.style.display = 'none';
    transactionReceipt.textContent = output;
  } catch (e) {
    console.error("Error setting attribute: ", e);
  }
}


async function init_registry() {
  if (!REGISTRY) {
    try {
      REGISTRY = await new Registry();
    } catch(e) {
      console.error(e);
    } finally {
      spinner.style.display = 'none';
    }
  }
}
