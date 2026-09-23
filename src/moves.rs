
/**
 * For now I am making the decision that we will pass
 * the cube by copy and return a new instance of cube.
 * This has performance implication that I will have to
 * deal with later.
 */

// Clockwise Moves (about the center of the cube)

fn U(cube: Cube) -> Cube {
    // a U is described by the following transformations
   // [27, 28, 29] moves to [0, 1, 2]
   // [0, 1, 2] moves to [18, 19, 20]
   // [18, 19, 20] moves to [9, 10, 11]
   // [9, 10, 11] moves to [27, 28, 29] 

   // construct a !vec[] of (l, r) and then proceed to swap (l, r)
   // we will 4 !vec in this case?

}

fn D(cube: Cube) -> Cube {

}

fn R(cube: Cube) -> Cube {

}

fn L(cube: Cube) -> Cube {

}

fn F(cube: Cube) -> Cube {

}

fn B(cube: Cube) -> Cube {

}


// Counter-Clockwise Moves (about the center of the cube)
//notation here is kinda garbage but the p means prime

fn Up(cube: Cube) -> Cube {

}

fn Dp(cube: Cube) -> Cube {

}

fn Rp(cube: Cube) -> Cube {

}

fn Lp(cube: Cube) -> Cube {

}

fn Fp(cube: Cube) -> Cube {

}

fn Bp(cube: Cube) -> Cube {

}


// slice moves

fn M(cube: Cube) -> Cube {

}

fn E(cube: Cube) -> Cube {

}

fn S(cube: Cube) -> Cube {

}