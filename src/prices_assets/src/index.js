import { prices } from "../../declarations/prices";

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the greet method
  const owner = await prices.get_owner();

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = owner;

  return false;
});
