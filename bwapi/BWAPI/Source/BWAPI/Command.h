#pragma once
class Command;
#include <BWAPI/Client/CommandTemp.h>
#include "UnitImpl.h"
#include "PlayerImpl.h"

namespace BWAPI
{
  using Command = CommandTemp<UnitImpl, PlayerImpl>;
}