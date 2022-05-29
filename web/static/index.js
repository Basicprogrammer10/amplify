const BADGE_TEXT = ["Not Started", "In Progress", "Complete"];
const BADGE_COLORS = [
  ["#475569", "#F1F5F9"],
  ["#CA8A04", "#FEF9C3"],
  ["#059669", "#D1FAE5"],
];
const LANG_IDENTIFIERS = {
  c: "ace/mode/c_cpp",
  "c++": "ace/mode/c_cpp",
  js: "ace/mode/javascript",
  python: "ace/mode/python",
  rust: "ace/mode/rust",
};

async function getSession() {
  let info = await (
    await fetch("http://localhost:8080/api/generic_self")
  ).json();

  if (!("error" in info)) return info;
  return null;
}

function redirect(path) {
  location = `${path}?${(Math.random() + 1).toString(36).substring(7)}`;
}

function statusBadgeColor(id) {
  let color = BADGE_COLORS[id];
  return `background: ${color[0]};color: ${color[1]};`;
}

async function run(lang, prob) {
  let resp = await (
    await fetch("http://localhost:8080/api/solve", {
      method: "POST",
      body: JSON.stringify({
        lang,
        problem: prob,
        code: editor.getValue(),
      }),
    })
  ).json();

  let stdout = Diff.diffLines(resp.expected, resp.stdout.trimEnd());
  let stdoutDiff = "";

  stdout.forEach((d) => {
    if (d.added) stdoutDiff += "<" + d.value.replace("\n", "\n<");
    else if (d.removed) stdoutDiff += ">" + d.value.replace("\n", "\n<");
    else stdoutDiff += d.value;
  });

  document.querySelector("[stderr]").innerHTML = resp.stderr;
  document.querySelector("[stdout]").innerHTML = stdoutDiff;
}
