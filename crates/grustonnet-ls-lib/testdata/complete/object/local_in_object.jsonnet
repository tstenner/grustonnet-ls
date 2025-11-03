local myVar = { key: 5 };

{
  local myLocal = myVar,
  x: myLocal,
  useMyVar: myVar,
}
