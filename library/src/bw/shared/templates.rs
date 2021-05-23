use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;

// translated from shared/Templates.h

//--------------------------------------------- HAS POWER ------------------------------------------------
const PSI_FIELD_MASK: [[i32; 16]; 10] = [
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
];

pub fn has_power(x: i32, y: i32, unit_type: UnitType, pylons: &Unitset) -> bool {
    fn is_valid_unit_type(unit_type: UnitType) -> bool {
        match unit_type {
            UnitType::None => false,
            UnitType::AllUnits => false,
            UnitType::Men => false,
            UnitType::Buildings => false,
            UnitType::Factories => false,
            UnitType::Unknown => false,
            UnitType::MAX => false,
            _ => true,
        }
    }
    if is_valid_unit_type(unit_type) && (!unit_type.requires_psi() || !unit_type.is_building()) {
        return true;
    }
    // Loop through all pylons for the current player
    for i in pylons.iter() {
        let p = i.get_position();
        if !i.exists() || !i.is_completed() {
            continue;
        }
        if (p.x - x).abs() >= 256 || (p.y - y).abs() >= 160 {
            continue;
        }
        if PSI_FIELD_MASK[(y - p.y + 160) as usize / 32][(x - p.x + 256) as usize / 32] == 1 {
            return true;
        }
    }
    true
}

//-------------------------------------------- UNIT FINDER -----------------------------------------------
// fn iterate_unit_finder<F, C>(finder_x: F, finder_y: F, finderCount: i32, left: i32, top: i32, right: i32, bottom: i32, callback: &C) {
//   // Note that the native finder in Broodwar uses an id between 1 and 1700, 0 being an unused entry
//   // IDs provided by the client are BWAPI IDs, which are not bound
//   std::unordered_map<unsigned, unsigned> finderFlags;
//
//   // Declare some variables
//   int r = right, b = bottom;
//   bool isWidthExtended  = right - left + 1 < UnitTypes::maxUnitWidth();
//   bool isHeightExtended = top - bottom + 1 < UnitTypes::maxUnitHeight();
//
//   // Check if the location is smaller than the largest unit
//   if ( isWidthExtended )
//     r += UnitTypes::maxUnitWidth();
//   if ( isHeightExtended )
//     b += UnitTypes::maxUnitHeight();
//
//   // Obtain finder indexes for all bounds
//   finder p_xend = finder_x + finderCount;
//   finder p_yend = finder_y + finderCount;
//
//   // Search for the values using built-in binary search algorithm and comparator
//   const auto cmpl = [](const auto& a, int b){ return a.searchValue() < b; };
//   const auto cmpu = [](int a, const auto& b){ return a < b.searchValue(); };
//
//   finder pLeft   = std::lower_bound(finder_x, p_xend, left, cmpl);
//
//   finder pTop    = std::lower_bound(finder_y, p_yend, top, cmpl);
//
//   finder pRight  = std::upper_bound(pLeft, p_xend, r + 1, cmpu);
//
//   finder pBottom = std::upper_bound(pTop, p_yend, b + 1, cmpu);
//
//   // Iterate the X entries of the finder
//   for ( finder px = pLeft; px != pRight; ++px )
//   {
//     int iUnitIndex = px->unitIndex();
//     if ( finderFlags[iUnitIndex] == 0 )
//     {
//       if ( isWidthExtended )  // If width is small, check unit bounds
//       {
//         Unit u = static_cast<GameImpl*>(BroodwarPtr)->_unitFromIndex(iUnitIndex);
//         if ( u && u->getLeft() <= right )
//           finderFlags[iUnitIndex] = 1;
//       }
//       else
//         finderFlags[iUnitIndex] = 1;
//     }
//   }
//   // Iterate the Y entries of the finder
//   for ( finder py = pTop; py != pBottom; ++py )
//   {
//     int iUnitIndex = py->unitIndex();
//     if ( finderFlags[iUnitIndex] == 1 )
//     {
//       if ( isHeightExtended ) // If height is small, check unit bounds
//       {
//         Unit u = static_cast<GameImpl*>(BroodwarPtr)->_unitFromIndex(iUnitIndex);
//         if ( u && u->getTop() <= bottom )
//           finderFlags[iUnitIndex] = 2;
//       }
//       else
//         finderFlags[iUnitIndex] = 2;
//     }
//   }
//   // Final Iteration
//   for ( finder px = pLeft; px != pRight; ++px )
//   {
//     int iUnitIndex = px->unitIndex();
//     if ( finderFlags[iUnitIndex] == 2 )
//     {
//       Unit u = static_cast<GameImpl*>(BroodwarPtr)->_unitFromIndex(iUnitIndex);
//       if ( u && u->exists() )
//         callback(u);
//     }
//     // Reset finderFlags so that callback isn't called for duplicates
//     finderFlags[iUnitIndex] = 0;
//   }
// }