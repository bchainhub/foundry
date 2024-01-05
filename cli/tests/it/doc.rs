use foxar_cli_test_utils::util::{setup_spark_remote, RemoteProject};

#[test]
fn can_generate_solmate_docs() {
    let (prj, _) =
        setup_spark_remote(RemoteProject::new("transmissions11/solmate").set_build(false));
    prj.spark_command()
        .args(["doc", "--build"])
        .ensure_execute_success()
        .expect("`spark doc` failed");
}
