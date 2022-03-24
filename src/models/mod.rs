define_evolves!(
    pub mod foo_model;
)

// emits two functions

// call from evolve_db
fn evolve() {
    evolver = Evolver{};
    for model in models {
        // can register to do proper sorting, just need info functions as part of evolve
        evolver.register(model);
    }
    for model in models {
        // basically remove FKs if necessary
        // remove whatever constraints we might need to break
        model::pre_evolve();
    }

    for model in models {
        // add/drop/whatever columns
        model::evolve();
    }

    for model in models {
        // add FKs
        // add back our constraints that should be upheld
        model::post_evolve();
    }
}

// smoke test in main
fn evolve_check(conn) {
    for model in models {
        model::first(conn) // smoke test
    }
}