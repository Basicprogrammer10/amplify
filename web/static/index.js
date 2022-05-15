async function getSession() {
  let info = await (
    await fetch("http://localhost:8080/api/generic_self")
  ).json();

  if (!("error" in info)) return info;
  return null;
}
