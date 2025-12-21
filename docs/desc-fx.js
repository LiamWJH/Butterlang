const desc = document.getElementById("desc-fx");

const texts = [
  "Fast.",
  "Simple.",
  "Rust-coded.",
  "Some BS.",
  "Transpiled.",
  "Reliable.",
];

let i = 0;
let j = 0;
let text = "";

function type() {
  text = texts[j % texts.length];
  desc.textContent = "BUTTER is: " + text.slice(0, i++);
  
  if (i <= text.length) {
    setTimeout(type, 120); // typing speed
  } else {
    setTimeout(() => {
      i = 0;
      j++;
      desc.textContent = "BUTTER is: ";
      type();
    }, 800);
  }
}

type();
