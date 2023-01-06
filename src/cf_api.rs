use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
/*
 * DO NOT EDIT THIS FILE MANUALLY
 * IT WAS GENERATED USING tools/struct_gen.py
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct SortableGameVersion {
    // Original version name (e.g. 1.5b)
    #[serde(rename = "gameVersionName")]
    game_version_name: String,
    // Used for sorting (e.g. 0000000001.0000000005)
    #[serde(rename = "gameVersionPadded")]
    game_version_padded: String,
    // game version clean name (e.g. 1.5)
    #[serde(rename = "gameVersion")]
    game_version: String,
    // Game version release date
    #[serde(rename = "gameVersionReleaseDate")]
    game_version_release_date: String,
    // Game version type id
    #[serde(rename = "gameVersionTypeId")]
    game_version_type_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    // A zero based index of the first item that is included in the response
    index: i64,
    // The requested number of items to be included in the response
    #[serde(rename = "pageSize")]
    page_size: i64,
    // The actual number of items that were included in the response
    #[serde(rename = "resultCount")]
    result_count: i64,
    // The total number of items available by the request
    #[serde(rename = "totalCount")]
    total_count: i64,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ModsSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ModLoaderInstallMethod {
    ForgeInstaller = 1,
    ForgeJarInstall = 2,
    ForgeInstallerV2 = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModLinks {
    #[serde(rename = "websiteUrl")]
    website_url: String,
    #[serde(rename = "wikiUrl")]
    wiki_url: String,
    #[serde(rename = "issuesUrl")]
    issues_url: String,
    #[serde(rename = "sourceUrl")]
    source_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModAuthor {
    id: i64,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModAsset {
    id: i64,
    #[serde(rename = "modId")]
    mod_id: i64,
    title: String,
    description: String,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    // The mod id
    id: i64,
    // The game id this mod is for
    #[serde(rename = "gameId")]
    game_id: i64,
    // The name of the mod
    name: String,
    // The mod slug that would appear in the URL
    slug: String,
    // Relevant links for the mod such as Issue tracker and Wiki
    links: ModLinks,
    // Mod summary
    summary: String,
    // Current mod status
    status: ModStatus,
    // Number of downloads for the mod
    #[serde(rename = "downloadCount")]
    download_count: i64,
    // Whether the mod is included in the featured mods list
    #[serde(rename = "isFeatured")]
    is_featured: bool,
    // The main category of the mod as it was chosen by the mod author
    #[serde(rename = "primaryCategoryId")]
    primary_category_id: i64,
    // List of categories that this mod is related to
    categories: Vec<Category>,
    // The class id this mod belongs to
    #[serde(rename = "classId")]
    class_id: Option<i64>,
    // List of the mod's authors
    authors: Vec<ModAuthor>,
    // The mod's logo asset
    logo: ModAsset,
    // List of screenshots assets
    screenshots: Vec<ModAsset>,
    // The id of the main file of the mod
    #[serde(rename = "mainFileId")]
    main_file_id: i64,
    // List of latest files of the mod
    #[serde(rename = "latestFiles")]
    latest_files: Vec<File>,
    // List of file related details for the latest files of the mod
    #[serde(rename = "latestFilesIndexes")]
    latest_files_indexes: Vec<FileIndex>,
    // The creation date of the mod
    #[serde(rename = "dateCreated")]
    date_created: String,
    // The last time the mod was modified
    #[serde(rename = "dateModified")]
    date_modified: String,
    // The release date of the mod
    #[serde(rename = "dateReleased")]
    date_released: String,
    // Is mod allowed to be distributed
    #[serde(rename = "allowModDistribution")]
    allow_mod_distribution: Option<bool>,
    // The mod popularity rank for the game
    #[serde(rename = "gamePopularityRank")]
    game_popularity_rank: i64,
    // Is the mod available for search. This can be false when a mod is experimental, in a deleted state or has only alpha files
    #[serde(rename = "isAvailable")]
    is_available: bool,
    // The mod's thumbs up count
    #[serde(rename = "thumbsUpCount")]
    thumbs_up_count: i64,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFingerprintMatchesRequestBody {
    fingerprints: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftGameVersion {
    id: i64,
    #[serde(rename = "gameVersionId")]
    game_version_id: i64,
    #[serde(rename = "versionString")]
    version_string: String,
    #[serde(rename = "jarDownloadUrl")]
    jar_download_url: String,
    #[serde(rename = "jsonDownloadUrl")]
    json_download_url: String,
    approved: bool,
    #[serde(rename = "dateModified")]
    date_modified: String,
    #[serde(rename = "gameVersionTypeId")]
    game_version_type_id: i64,
    // None
    #[serde(rename = "gameVersionStatus")]
    game_version_status: GameVersionStatus,
    // None
    #[serde(rename = "gameVersionTypeStatus")]
    game_version_type_status: GameVersionTypeStatus,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CoreStatus {
    Draft = 1,
    Test = 2,
    PendingReview = 3,
    Rejected = 4,
    Approved = 5,
    Live = 6,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GameVersionTypeStatus {
    Normal = 1,
    Deleted = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameVersionType {
    id: i64,
    #[serde(rename = "gameId")]
    game_id: i64,
    name: String,
    slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    // The category id
    id: i64,
    // The game id related to the category
    #[serde(rename = "gameId")]
    game_id: i64,
    // Category name
    name: String,
    // The category slug as it appear in the URL
    slug: String,
    // The category URL
    url: String,
    // URL for the category icon
    #[serde(rename = "iconUrl")]
    icon_url: String,
    // Last modified date of the category
    #[serde(rename = "dateModified")]
    date_modified: String,
    // A top level category for other categories
    #[serde(rename = "isClass")]
    is_class: Option<bool>,
    // The class id of the category, meaning - the class of which this category is under
    #[serde(rename = "classId")]
    class_id: Option<i64>,
    // The parent category for this category
    #[serde(rename = "parentCategoryId")]
    parent_category_id: Option<i64>,
    // The display index for this category
    #[serde(rename = "displayIndex")]
    display_index: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDependency {
    #[serde(rename = "modId")]
    mod_id: i64,
    // None
    #[serde(rename = "relationType")]
    relation_type: FileRelationType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseOfListOfMinecraftGameVersion {
    // The response data
    data: Vec<MinecraftGameVersion>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseOfListOfMinecraftModLoaderIndex {
    // The response data
    data: Vec<MinecraftModLoaderIndex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftModLoaderIndex {
    name: String,
    #[serde(rename = "gameVersion")]
    game_version: String,
    latest: bool,
    recommended: bool,
    #[serde(rename = "dateModified")]
    date_modified: String,
    // None
    #[serde(rename = "type")]
    type_type: ModLoaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    // The file id
    id: i64,
    // The game id related to the mod that this file belongs to
    #[serde(rename = "gameId")]
    game_id: i64,
    // The mod id
    #[serde(rename = "modId")]
    mod_id: i64,
    // Whether the file is available to download
    #[serde(rename = "isAvailable")]
    is_available: bool,
    // Display name of the file
    #[serde(rename = "displayName")]
    display_name: String,
    // Exact file name
    #[serde(rename = "fileName")]
    file_name: String,
    // The file release type
    #[serde(rename = "releaseType")]
    release_type: FileReleaseType,
    // Status of the file
    #[serde(rename = "fileStatus")]
    file_status: FileStatus,
    // The file hash (i.e. md5 or sha1)
    hashes: Vec<FileHash>,
    // The file timestamp
    #[serde(rename = "fileDate")]
    file_date: String,
    // The file length in bytes
    #[serde(rename = "fileLength")]
    file_length: i64,
    // The number of downloads for the file
    #[serde(rename = "downloadCount")]
    download_count: i64,
    // The file download URL
    #[serde(rename = "downloadUrl")]
    download_url: String,
    // List of game versions this file is relevant for
    #[serde(rename = "gameVersions")]
    game_versions: Vec<String>,
    // Metadata used for sorting by game versions
    #[serde(rename = "sortableGameVersions")]
    sortable_game_versions: Vec<SortableGameVersion>,
    // List of dependencies files
    dependencies: Vec<FileDependency>,
    #[serde(rename = "exposeAsAlternative")]
    expose_as_alternative: Option<bool>,
    #[serde(rename = "parentProjectFileId")]
    parent_project_file_id: Option<i64>,
    #[serde(rename = "alternateFileId")]
    alternate_file_id: Option<i64>,
    #[serde(rename = "isServerPack")]
    is_server_pack: Option<bool>,
    #[serde(rename = "serverPackFileId")]
    server_pack_file_id: Option<i64>,
    #[serde(rename = "fileFingerprint")]
    file_fingerprint: i64,
    modules: Vec<FileModule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftModLoaderVersion {
    id: i64,
    #[serde(rename = "gameVersionId")]
    game_version_id: i64,
    #[serde(rename = "minecraftGameVersionId")]
    minecraft_game_version_id: i64,
    #[serde(rename = "forgeVersion")]
    forge_version: String,
    name: String,
    // None
    #[serde(rename = "type")]
    type_type: ModLoaderType,
    #[serde(rename = "downloadUrl")]
    download_url: String,
    filename: String,
    // None
    #[serde(rename = "installMethod")]
    install_method: ModLoaderInstallMethod,
    latest: bool,
    recommended: bool,
    approved: bool,
    #[serde(rename = "dateModified")]
    date_modified: String,
    #[serde(rename = "mavenVersionString")]
    maven_version_string: String,
    #[serde(rename = "versionJson")]
    version_json: String,
    #[serde(rename = "librariesInstallLocation")]
    libraries_install_location: String,
    #[serde(rename = "minecraftVersion")]
    minecraft_version: String,
    #[serde(rename = "additionalFilesJson")]
    additional_files_json: String,
    #[serde(rename = "modLoaderGameVersionId")]
    mod_loader_game_version_id: i64,
    #[serde(rename = "modLoaderGameVersionTypeId")]
    mod_loader_game_version_type_id: i64,
    // None
    #[serde(rename = "modLoaderGameVersionStatus")]
    mod_loader_game_version_status: GameVersionStatus,
    // None
    #[serde(rename = "modLoaderGameVersionTypeStatus")]
    mod_loader_game_version_type_status: GameVersionTypeStatus,
    #[serde(rename = "mcGameVersionId")]
    mc_game_version_id: i64,
    #[serde(rename = "mcGameVersionTypeId")]
    mc_game_version_type_id: i64,
    // None
    #[serde(rename = "mcGameVersionStatus")]
    mc_game_version_status: GameVersionStatus,
    // None
    #[serde(rename = "mcGameVersionTypeStatus")]
    mc_game_version_type_status: GameVersionTypeStatus,
    #[serde(rename = "installProfileJson")]
    install_profile_json: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FingerprintsMatchesResult {
    #[serde(rename = "isCacheBuilt")]
    is_cache_built: bool,
    #[serde(rename = "exactMatches")]
    exact_matches: Vec<FingerprintMatch>,
    #[serde(rename = "exactFingerprints")]
    exact_fingerprints: Vec<i64>,
    #[serde(rename = "partialMatches")]
    partial_matches: Vec<FingerprintMatch>,
    #[serde(rename = "partialMatchFingerprints")]
    partial_match_fingerprints: serde_json::Value,
    #[serde(rename = "None")]
    none: Vec<i64>,
    #[serde(rename = "installedFingerprints")]
    installed_fingerprints: Vec<i64>,
    #[serde(rename = "unmatchedFingerprints")]
    unmatched_fingerprints: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFuzzyMatchesRequestBody {
    #[serde(rename = "gameId")]
    game_id: i64,
    fingerprints: Vec<FolderFingerprint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetModFilesRequestBody {
    #[serde(rename = "fileIds")]
    file_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileIndex {
    #[serde(rename = "gameVersion")]
    game_version: String,
    #[serde(rename = "fileId")]
    file_id: i64,
    filename: String,
    // None
    #[serde(rename = "releaseType")]
    release_type: FileReleaseType,
    #[serde(rename = "gameVersionTypeId")]
    game_version_type_id: Option<i64>,
    // None
    #[serde(rename = "modLoader")]
    mod_loader: ModLoaderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileHash {
    value: String,
    // None
    algo: HashAlgo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FingerprintMatch {
    id: i64,
    file: File,
    #[serde(rename = "latestFiles")]
    latest_files: Vec<File>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GameVersionStatus {
    Approved = 1,
    Deleted = 2,
    New = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetFeaturedModsRequestBody {
    #[serde(rename = "gameId")]
    game_id: i64,
    #[serde(rename = "excludedModIds")]
    excluded_mod_ids: Vec<i64>,
    #[serde(rename = "gameVersionTypeId")]
    game_version_type_id: Option<i64>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FolderFingerprint {
    foldername: String,
    fingerprints: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FingerprintFuzzyMatchResult {
    #[serde(rename = "fuzzyMatches")]
    fuzzy_matches: Vec<FingerprintFuzzyMatch>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CoreApiStatus {
    Private = 1,
    Public = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileModule {
    name: String,
    fingerprint: i64,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseOfMinecraftGameVersion {
    // The response data
    data: MinecraftGameVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturedModsResponse {
    featured: Vec<Mod>,
    popular: Vec<Mod>,
    #[serde(rename = "recentlyUpdated")]
    recently_updated: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameAssets {
    #[serde(rename = "iconUrl")]
    icon_url: String,
    #[serde(rename = "tileUrl")]
    tile_url: String,
    #[serde(rename = "coverUrl")]
    cover_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: i64,
    name: String,
    slug: String,
    #[serde(rename = "dateModified")]
    date_modified: String,
    assets: GameAssets,
    // None
    status: CoreStatus,
    // None
    #[serde(rename = "apiStatus")]
    api_status: CoreApiStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseOfMinecraftModLoaderVersion {
    // The response data
    data: MinecraftModLoaderVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameVersionsByType {
    #[serde(rename = "type")]
    type_type: i64,
    versions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FingerprintFuzzyMatch {
    id: i64,
    file: File,
    #[serde(rename = "latestFiles")]
    latest_files: Vec<File>,
    fingerprints: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetModsByIdsListRequestBody {
    #[serde(rename = "modIds")]
    mod_ids: Vec<i64>,
}


  #[derive(Serialize, Deserialize, Debug)]
pub struct Response < T > {
data: T,
}