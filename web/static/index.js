const TIME_UNITS = [
  ["sec", 60],
  ["min", 60],
  ["hour", 24],
  ["day", 30],
  ["month", 12],
  ["year", 0],
];
const MONTHS = [
  "January",
  "February",
  "March",
  "April",
  "May",
  "June",
  "July",
  "August",
  "September",
  "October",
  "November",
  "December",
];
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
const COMPLETE_MESSAGES = [
  "🎊 Congratulations! You completed this problem!",
  "✨ very beans!!! You got it!",
  "🧊 cool beans,,, You got it!",
  "🥭 go go mango,,, You got it!",
];
const FAIL_MESSAGES = [
  "hm, not quite",
  "not quite, try again",
  "*incorrect*",
  "keep trying, i believe in you",
  "never give up",
  "you're getting close, i can feel it",
];

async function getSession() {
  let info = await (await fetch("/api/generic_self")).json();
  window.startLang = info.lang ?? "rust";

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
    await fetch("/api/solve", {
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
    else if (d.removed) stdoutDiff += ">" + d.value.split("\n").join("\n>");
    else stdoutDiff += d.value;
  });

  document.querySelector("[stderr]").innerHTML = resp.stderr;
  document.querySelector("[stdout]").innerHTML = stdoutDiff;

  if (resp.success) {
    document.querySelector("[finish-box]").classList.remove("hidden");
    document.querySelector("[complete-message]").innerHTML =
      COMPLETE_MESSAGES[[Math.floor(Math.random() * COMPLETE_MESSAGES.length)]];
    document.querySelector("[problem-badge]").style = statusBadgeColor(2);
    document.querySelector("[problem-badge]").innerHTML = BADGE_TEXT[2];
    return 2;
  }

  document.querySelector("[finish-box]").classList.remove("hidden");
  document.querySelector("[complete-message]").innerHTML =
    FAIL_MESSAGES[[Math.floor(Math.random() * FAIL_MESSAGES.length)]];
  return 1;
}

function bestTime(time) {
  for (let e = 0; e < TIME_UNITS.length; e++) {
    const i = TIME_UNITS[e];

    if (i[1] == 0 || time < i[1]) {
      time = Math.round(time);
      return `${time} ${i[0]}${time > 1 ? "s" : ""}`;
    }

    time /= i[1];
  }

  return `${Math.round(time)} years`;
}