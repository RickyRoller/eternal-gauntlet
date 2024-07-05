export function send_to_js(score) {
  console.log("score sent to js", score);
  window.postMessage({ type: "score", score }, "*");
}
