mod application;
mod domain;
mod infrastructure;
mod interfaces;

use crate::application::usecase::load_static_data::LoadStaticDataUsecase;
use crate::infrastructure::persistence::file_ability_repository::FileAbilityRepository;
use crate::infrastructure::persistence::file_item_repository::FileItemRepository;
use crate::infrastructure::persistence::file_move_repository::FileMoveRepository;
use crate::infrastructure::persistence::file_pokemon_repository::FilePokemonRepository;
use crate::infrastructure::persistence::file_pokemon_species_repository::FilePokemonSpeciesRepository;
use crate::infrastructure::persistence::file_type_repository::FileTypeRepository;
use log::info;

fn main() {
    // ロガーの初期化
    env_logger::init();

    info!("アプリケーションを開始します。");

    // 1. 具体的なリポジトリ実装のインスタンス化
    let item_repository = FileItemRepository;
    let pokemon_repository = FilePokemonRepository;
    let move_repository = FileMoveRepository;
    let pokemon_species_repository = FilePokemonSpeciesRepository;
    let type_repository = FileTypeRepository;
    let ability_repository = FileAbilityRepository;

    // 2. ユースケースにリポジトリを注入（DI: Dependency Injection）
    let load_static_data_use_case = LoadStaticDataUsecase::new(
        item_repository,
        pokemon_repository,
        move_repository,
        pokemon_species_repository,
        type_repository,
        ability_repository,
    );

    // 3. ユースケースの実行
    let loaded_data = load_static_data_use_case.execute();

    info!("ロードされたアイテムの数: {}", loaded_data.items.len());
    if let Some(sample_item) = loaded_data.items.first() {
        info!(
            "アイテムサンプル出力: ID: {}, 名前: {}",
            sample_item.id,
            sample_item.name
        );
    }

    info!("ロードされたポケモンの数: {}", loaded_data.pokemons.len());
    if let Some(sample_pokemon) = loaded_data.pokemons.first() {
        info!(
            "ポケモンサンプル出力: ID: {}, 名前: {}",
            sample_pokemon.id,
            sample_pokemon.name
        );
    }

    info!("ロードされた技の数: {}", loaded_data.moves.len());
    if let Some(sample_move) = loaded_data.moves.first() {
        info!(
            "技サンプル出力: ID: {}, 名前: {}",
            sample_move.id,
            sample_move.name
        );
    }

    info!(
        "ロードされたポケモンの種類の数: {}",
        loaded_data.pokemon_species.len()
    );
    if let Some(sample_species) = loaded_data.pokemon_species.first() {
        info!(
            "ポケモン種類サンプル出力: ID: {}, 名前: {}",
            sample_species.id,
            sample_species.name
        );
    }

    info!("ロードされたタイプの数: {}", loaded_data.types.len());
    if let Some(sample_type) = loaded_data.types.first() {
        info!(
            "タイプサンプル出力: ID: {}, 名前: {}",
            sample_type.id,
            sample_type.name
        );
    }

    info!("ロードされた特性の数: {}", loaded_data.abilities.len());
    if let Some(sample_ability) = loaded_data.abilities.first() {
        info!(
            "特性サンプル出力: ID: {}, 名前: {}",
            sample_ability.id,
            sample_ability.name
        );
    }

    info!("アプリケーションを終了します。");
}
