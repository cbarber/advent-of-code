f = io.open("./input", "r");

local elves = {};
local i = 1;
elves[i] = 0
while true do
  local line = f:read();
  if line == nil then break end

  if #line == 0 then
    i = i + 1;
    elves[i] = 0;
  else
    elves[i] = elves[i] + tonumber(line);
  end
end

local max = 0;
for _, value in pairs(elves) do
  max = math.max(max, value)
end

print(max)
