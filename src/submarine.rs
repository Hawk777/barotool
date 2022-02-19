use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use strong_xml::{XmlRead, XmlWrite};
use libflate::gzip::{Decoder, Encoder};

/// A boolean which is stored in XML with a leading capital letter.
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
struct CapitalBool(pub bool);

impl std::fmt::Display for CapitalBool {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		if self.0 {
			write!(f, "True")
		} else {
			write!(f, "False")
		}
	}
}

impl std::str::FromStr for CapitalBool {
	type Err = std::io::Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"True" => Ok(Self(true)),
			"False" => Ok(Self(false)),
			_ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid CapitalBool value {}", s))),
		}
	}
}

/// A submarine.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Submarine")]
struct Submarine {
	#[xml(attr = "description")]
	pub description: String,
	#[xml(attr = "checkval")]
	pub check_val: u32,
	#[xml(attr = "price")]
	pub price: u32,
	#[xml(attr = "initialsuppliesspawned")]
	pub initial_supplies_spawned: bool,
	#[xml(attr = "type")]
	pub submarine_type: String,
	#[xml(attr = "class")]
	pub class: String,
	#[xml(attr = "tags")]
	pub tags: String,
	#[xml(attr = "gameversion")]
	pub game_version: String,
	#[xml(attr = "dimensions")]
	pub dimensions: String,
	#[xml(attr = "cargocapacity")]
	pub cargo_capacity: u32,
	#[xml(attr = "recommendedcrewsizemin")]
	pub recommended_crew_size_min: u32,
	#[xml(attr = "recommendedcrewsizemax")]
	pub recommended_crew_size_max: u32,
	#[xml(attr = "recommendedcrewexperience")]
	pub recommended_crew_experience: String,
	#[xml(attr = "requiredcontentpackages")]
	pub required_content_packages: String,
	#[xml(attr = "name")]
	pub name: String,
	#[xml(child = "Item")]
	pub child: Vec<Item>,
	#[xml(child = "WayPoint")]
	pub waypoint: Vec<Waypoint>,
	#[xml(child = "LinkedSubmarine")]
	pub linked_submarine: Vec<LinkedSubmarine>,
}

/// An item inside a submarine.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Item")]
struct Item {
	#[xml(attr = "name")]
	pub name: String,
	#[xml(attr = "identifier")]
	pub identifier: String,
	#[xml(attr = "ID")]
	pub id: u32,
	#[xml(attr = "flippedx")]
	pub flipped_x: Option<bool>,
	#[xml(attr = "flippedy")]
	pub flipped_y: Option<bool>,
	#[xml(attr = "rect")]
	pub rect: String,
	#[xml(attr = "noninteractable")]
	pub non_interactable: CapitalBool,
	#[xml(attr = "nonplayerteaminteractable")]
	pub non_player_team_interactable: CapitalBool,
	#[xml(attr = "allowswapping")]
	pub allow_swapping: CapitalBool,
	#[xml(attr = "rotation")]
	pub rotation: f32,
	#[xml(attr = "scale")]
	pub scale: f32,
	#[xml(attr = "spritecolor")]
	pub sprite_color: String,
	#[xml(attr = "inventoryiconcolor")]
	pub inventory_icon_color: String,
	#[xml(attr = "containercolor")]
	pub container_color: String,
	#[xml(attr = "condition")]
	pub condition: f32,
	#[xml(attr = "invulnerabletodamage")]
	pub invulnerable_to_damage: CapitalBool,
	#[xml(attr = "tags")]
	pub tags: String,
	#[xml(attr = "displaysidebysidewhenlinked")]
	pub display_side_by_side_when_linked: CapitalBool,
	#[xml(attr = "disallowedupgrades")]
	pub disallowed_upgrades: String,
	#[xml(attr = "spritedepth")]
	pub sprite_depth: f32,
	#[xml(attr = "hiddeningame")]
	pub hidden_in_game: CapitalBool,

	#[xml(child = "ConnectionPanel")]
	pub connection_panel: Option<ConnectionPanel>,
	#[xml(child = "Holdable")]
	pub holdable: Option<Holdable>,
	#[xml(child = "ItemContainer")]
	pub item_container: Option<ItemContainer>,
	#[xml(child = "LightComponent")]
	pub light_component: Option<LightComponent>,
	#[xml(child = "MeleeWeapon")]
	pub melee_weapon: Option<MeleeWeapon>,
	#[xml(child = "Pickable")]
	pub pickable: Option<Pickable>,
	#[xml(child = "Powered")]
	pub powered: Option<Powered>,
	#[xml(child = "Projectile")]
	pub projectile: Option<Projectile>,
	#[xml(child = "StatusHUD")]
	pub status_hud: Option<StatusHUD>,
	#[xml(child = "Throwable")]
	pub throwable: Option<Throwable>,
	#[xml(child = "Wearable")]
	pub wearable: Option<Wearable>,
	#[xml(child = "WifiComponent")]
	pub wifi_component: Option<WifiComponent>,
	#[xml(child = "Wire")]
	pub wire: Option<Wire>,
}

/// Information about a connection panel.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "ConnectionPanel")]
struct ConnectionPanel {
	#[xml(attr = "locked")]
	pub locked: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(child = "requireditem")]
	pub required_item: Option<RequiredItem>,
	#[xml(child = "input")]
	pub input: Vec<Input>,
	#[xml(child = "output")]
	pub output: Vec<Output>,
}

/// Information about an input connection.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "input")]
struct Input {
	#[xml(child = "link")]
	pub link: Vec<Link>,
}

/// Information about an input connection.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "output")]
struct Output {
	#[xml(child = "link")]
	pub link: Vec<Link>,
}

/// Information about a link to another component.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "link")]
struct Link {
	#[xml(attr = "w")]
	pub wire: u32,
}

/// Information about a holdable item.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Holdable")]
struct Holdable {
	#[xml(attr = "aimable")]
	pub aimable: Option<CapitalBool>,
	#[xml(attr = "holdpos")]
	pub hold_pos: String,
	#[xml(attr = "holdangle")]
	pub hold_angle: f32,
	#[xml(attr = "swingamount")]
	pub swing_amount: String,
	#[xml(attr = "swingspeed")]
	pub swing_speed: f32,
	#[xml(attr = "swingwhenholding")]
	pub swing_when_holding: CapitalBool,
	#[xml(attr = "swingwhenaiming")]
	pub swing_when_aiming: CapitalBool,
	#[xml(attr = "swingwhenusing")]
	pub swing_when_using: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(child = "requireditem")]
	pub required_item: Vec<RequiredItem>,
}

/// Information about a container.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "ItemContainer")]
struct ItemContainer {
	#[xml(attr = "containablerestrictions")]
	pub containable_restrictions: String,
	#[xml(attr = "autofill")]
	pub auto_fill: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(attr = "contained")]
	pub contained: String,
}

/// Information about a light component.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "LightComponent")]
struct LightComponent {
	#[xml(attr = "range")]
	pub range: f32,
	#[xml(attr = "castshadows")]
	pub cast_shadows: CapitalBool,
	#[xml(attr = "drawbehindsubs")]
	pub draw_behind_subs: CapitalBool,
	#[xml(attr = "ison")]
	pub is_on: CapitalBool,
	#[xml(attr = "flicker")]
	pub flicker: f32,
	#[xml(attr = "flickerspeed")]
	pub flicker_speed: f32,
	#[xml(attr = "pulsefrequency")]
	pub pulse_frequency: f32,
	#[xml(attr = "pulseamount")]
	pub pulse_amount: f32,
	#[xml(attr = "blinkfrequency")]
	pub blink_frequency: f32,
	#[xml(attr = "lightcolor")]
	pub light_color: String,
	#[xml(attr = "minvoltage")]
	pub min_voltage: f32,
	#[xml(attr = "powerconsumption")]
	pub power_consumption: f32,
	#[xml(attr = "voltage")]
	pub voltage: Option<f32>,
	#[xml(attr = "vulnerabletoemp")]
	pub vulnerable_to_emp: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about a mélée weapon.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "MeleeWeapon")]
struct MeleeWeapon {
	#[xml(attr = "holdpos")]
	pub hold_pos: String,
	#[xml(attr = "holdangle")]
	pub hold_angle: f32,
	#[xml(attr = "swingamount")]
	pub swing_amount: String,
	#[xml(attr = "swingspeed")]
	pub swing_speed: f32,
	#[xml(attr = "swingwhenholding")]
	pub swing_when_holding: CapitalBool,
	#[xml(attr = "swingwhenaiming")]
	pub swing_when_aiming: CapitalBool,
	#[xml(attr = "swingwhenusing")]
	pub swing_when_using: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about a pickable object.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Pickable")]
struct Pickable {
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about a powered item.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Powered")]
struct Powered {
	#[xml(attr = "minvoltage")]
	pub min_voltage: f32,
	#[xml(attr = "powerconsumption")]
	pub power_consumption: f32,
	#[xml(attr = "isactive")]
	pub is_active: CapitalBool,
	#[xml(attr = "currpowerconsumption")]
	pub curr_power_consumption: f32,
	#[xml(attr = "voltage")]
	pub voltage: Option<f32>,
	#[xml(attr = "vulnerabletoemp")]
	pub vulnerable_to_emp: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about a projectile.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Projectile")]
struct Projectile {
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about a status HUD.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "StatusHUD")]
struct StatusHUD {
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about an object that can be thrown.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Throwable")]
struct Throwable {
	#[xml(attr = "holdpos")]
	pub hold_pos: String,
	#[xml(attr = "holdangle")]
	pub hold_angle: f32,
	#[xml(attr = "swingamount")]
	pub swing_amount: String,
	#[xml(attr = "swingspeed")]
	pub swing_speed: f32,
	#[xml(attr = "swingwhenholding")]
	pub swing_when_holding: CapitalBool,
	#[xml(attr = "swingwhenaiming")]
	pub swing_when_aiming: CapitalBool,
	#[xml(attr = "swingwhenusing")]
	pub swing_when_using: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
}

/// Information about something that can be worn.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Wearable")]
struct Wearable {
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(attr = "variant")]
	pub variant: u32,
}

/// Information about a wifi communication component.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "WifiComponent")]
struct WifiComponent {
	#[xml(attr = "teamid")]
	pub team_id: String,
	#[xml(attr = "range")]
	pub range: f32,
	#[xml(attr = "channel")]
	pub channel: u32,
	#[xml(attr = "allowcrossteamcommunication")]
	pub allow_cross_team_communication: CapitalBool,
	#[xml(attr = "linktochat")]
	pub link_to_chat: CapitalBool,
	#[xml(attr = "minchatmessageinterval")]
	pub min_chat_message_interval: f32,
	#[xml(attr = "discardduplicatechatmessages")]
	pub discard_duplicate_chat_messages: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(attr = "channelmemory")]
	pub channel_memory: String,
}

/// Information about a wire.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "Wire")]
struct Wire {
	#[xml(attr = "noautolock")]
	pub no_auto_lock: CapitalBool,
	#[xml(attr = "usespritedepth")]
	pub use_sprite_depth: CapitalBool,
	#[xml(attr = "pickingtime")]
	pub picking_time: f32,
	#[xml(attr = "canbepicked")]
	pub can_be_picked: CapitalBool,
	#[xml(attr = "allowingameediting")]
	pub allow_in_game_editing: CapitalBool,
	#[xml(attr = "msg")]
	pub msg: String,
	#[xml(attr = "nodes")]
	pub nodes: Option<String>,
}

/// Information about an item required to perform an action.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "requireditem")]
struct RequiredItem {
	#[xml(attr = "items")]
	pub items: String,
	#[xml(attr = "type")]
	pub item_type: String,
	#[xml(attr = "optional")]
	pub optional: bool,
	#[xml(attr = "ignoreineditor")]
	pub ignore_in_editor: bool,
	#[xml(attr = "excludebroken")]
	pub exclude_broken: bool,
}

/// A waypoint.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "WayPoint")]
struct Waypoint {
	#[xml(attr = "ID")]
	pub id: u32,
	#[xml(attr = "x")]
	pub x: i32,
	#[xml(attr = "y")]
	pub y: i32,
	#[xml(attr = "spawn")]
	pub spawn: String,
	#[xml(attr = "idcardtags")]
	pub id_card_tags: Option<String>,
	#[xml(attr = "job")]
	pub job: Option<String>,
	#[xml(attr = "ladders")]
	pub ladders: Option<u32>,
	#[xml(attr = "gap")]
	pub gap: Option<u32>,
	// This is horrid, but I don’t know how to convince strong-xml to do anything else without
	// turning the entire element into a custom thing, which would be annoying.
	#[xml(attr = "linkedto0")]
	pub linked_to_0: Option<u32>,
	#[xml(attr = "linkedto1")]
	pub linked_to_1: Option<u32>,
	#[xml(attr = "linkedto2")]
	pub linked_to_2: Option<u32>,
	#[xml(attr = "linkedto3")]
	pub linked_to_3: Option<u32>,
	#[xml(attr = "linkedto4")]
	pub linked_to_4: Option<u32>,
	#[xml(attr = "linkedto5")]
	pub linked_to_5: Option<u32>,
	#[xml(attr = "linkedto6")]
	pub linked_to_6: Option<u32>,
	#[xml(attr = "linkedto7")]
	pub linked_to_7: Option<u32>,
	#[xml(attr = "linkedto8")]
	pub linked_to_8: Option<u32>,
	#[xml(attr = "linkedto9")]
	pub linked_to_9: Option<u32>,
}

/// A shuttle or drone.
#[derive(Debug, XmlRead, XmlWrite)]
#[xml(tag = "LinkedSubmarine")]
struct LinkedSubmarine {
	#[xml(attr = "name")]
	pub name: String,
	#[xml(attr = "description")]
	pub description: String,
	#[xml(attr = "checkval")]
	pub check_val: u32,
	#[xml(attr = "price")]
	pub price: u32,
	#[xml(attr = "initialsuppliesspawned")]
	pub initial_supplies_spawned: bool,
	#[xml(attr = "type")]
	pub submarine_type: String,
	#[xml(attr = "tags")]
	pub tags: String,
	#[xml(attr = "gameversion")]
	pub game_version: String,
	#[xml(attr = "dimensions")]
	pub dimensions: String,
	#[xml(attr = "cargocapacity")]
	pub cargo_capacity: u32,
	#[xml(attr = "recommendedcrewsizemin")]
	pub recommended_crew_size_min: u32,
	#[xml(attr = "recommendedcrewsizemax")]
	pub recommended_crew_size_max: u32,
	#[xml(attr = "recommendedcrewexperience")]
	pub recommended_crew_experience: String,
	#[xml(attr = "requiredcontentpackages")]
	pub required_content_packages: String,
	#[xml(attr = "originallinkedto")]
	pub original_linked_to: u32,
	#[xml(attr = "originalmyport")]
	pub original_my_port: u32,
	#[xml(attr = "pos")]
	pub pos: String,
	#[xml(child = "Item")]
	pub child: Vec<Item>,
	#[xml(child = "WayPoint")]
	pub waypoint: Vec<Waypoint>,
	#[xml(child = "LinkedSubmarine")]
	pub linked_submarine: Vec<LinkedSubmarine>,
}

/// The UTF-8 “BOM” (not really) which appears at the start of a submarine XML file.
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Given a string, returns an [I/O error](std::io::Error) of the [`InvalidData`
/// kind](std::io::ErrorKind::InvalidData) with that string as its message.
fn invalid_data(s: impl AsRef<str>) -> std::io::Error {
	std::io::Error::new(std::io::ErrorKind::InvalidData, s.as_ref())
}

/// Converts a [serde_xml_rs::Error](serde_xml_rs::Error) into an [std::io::Error](std::io::Error).
fn convert_error(e: strong_xml::XmlError) -> std::io::Error {
	match e {
		strong_xml::XmlError::IO(e) => e,
		e => std::io::Error::new(std::io::ErrorKind::InvalidData, e),
	}
}

/// Reads a submarine file into a parsed data structure.
fn load_submarine(filename: &OsStr) -> Result<Submarine> {
	let mut reader = BufReader::new(Decoder::new(BufReader::new(File::open(filename)?))?);
	let mut bom_buffer = [0_u8; UTF8_BOM.len()];
	reader.read_exact(&mut bom_buffer)?;
	if bom_buffer != UTF8_BOM {
		return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Expected UTF-8 BOM"));
	}
	let mut contents = vec![];
	reader.read_to_end(&mut contents)?;
	let contents = String::from_utf8(contents).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
	Submarine::from_str(&contents).map_err(convert_error)
}

/// Writes a parsed data structure into a submarine file.
fn save_submarine(filename: &OsStr, submarine: &Submarine) -> Result<()> {
	let mut writer = BufWriter::new(Encoder::new(BufWriter::new(File::create(filename)?))?);
	writer.write_all(&UTF8_BOM)?;
	let mut writer = strong_xml::XmlWriter::new(writer);
	submarine.to_writer(&mut writer).map_err(convert_error)?;
	let writer = writer.into_inner();
	let writer = writer.into_inner()?;
	let writer = writer.finish().into_result()?;
	let writer = writer.into_inner()?;
	writer.sync_all()?;
	Ok(())
}

/*
/// Reads a submarine file into an XML node tree.
fn load_submarine(filename: &OsStr) -> Result<Element> {
	let mut reader = BufReader::new(Decoder::new(BufReader::new(File::open(filename)?))?);
	let mut bom_buffer = [0_u8; UTF8_BOM.len()];
	reader.read_exact(&mut bom_buffer)?;
	if bom_buffer != UTF8_BOM {
		return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Expected UTF-8 BOM"));
	}
	let elt = Element::parse(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
	if elt.name == "Submarine" {
		Ok(elt)
	} else {
		Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Expected root element Submarine, got {}", elt.name)))
	}
}

/// Writes an XML node tree into a submarine file.
fn save_submarine(filename: &OsStr, submarine: &Element) -> Result<()> {
	let mut writer = BufWriter::new(Encoder::new(BufWriter::new(File::create(filename)?))?);
	writer.write_all(&UTF8_BOM)?;
	submarine.write_with_config(&mut writer, xmltree::EmitterConfig::new().perform_indent(true)).map_err(|e| match e {
		xmltree::Error::Io(e) => e,
		other => Err(other).unwrap(),
	})?;
	let writer = writer.into_inner()?;
	let writer = writer.finish().into_result()?;
	let writer = writer.into_inner()?;
	writer.sync_all()?;
	Ok(())
}

/// Information about an item.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Item<'a> {
	/// The `Item` XML element.
	pub xml: &'a Element,

	/// The item ID number.
	pub id: u64,

	/// The type of item.
	pub identifier: &'a str,

	/// The geometric coordinates of the item.
	pub rect: (i64, i64, i64, i64),

	/// Information about the container that this item is, if it is one.
	pub container: Option<Container<'a>>,
}

impl<'a> Item<'a> {
	/// Parse information about an item from an XML element.
	pub fn parse(elt: &'a Element) -> Result<Self> {
		if elt.name != "Item" {
			return Err(invalid_data(format!("Expected Item element but got {}", elt.name)));
		}

		let id = elt.attributes.get("ID").ok_or_else(|| invalid_data("Item element is missing ID attribute"))?;
		let id = id.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

		let identifier = elt.attributes.get("identifier").ok_or_else(|| invalid_data("Item element is missing identifier attribute"))?;

		let rect = elt.attributes.get("rect").ok_or_else(|| invalid_data("Item element is missing rect attribute"))?;
		let rect = rect.split(',').map(|i| i.parse()).collect::<std::result::Result<Vec<i64>, std::num::ParseIntError>>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
		let rect = if rect.len() == 4 {
			Ok((rect[0], rect[1], rect[2], rect[3]))
		} else {
			Err(invalid_data(format!("rect attribute should have 4 comma-separated components, but found {}", rect.len())))
		}?;

		let item_container_elt = elt.children.iter().flat_map(XMLNode::as_element).find(|child| child.name == "ItemContainer");
		let container = item_container_elt.map(Container::parse).transpose()?;

		Ok(Self {
			xml: elt,
			id,
			identifier,
			rect,
			container,
		})
	}
}

/// Information about a container.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Container<'a> {
	/// The `ItemContainer` XML element.
	pub xml: &'a Element,

	/// The IDs of the items contained within the container.
	pub contents: Vec<Option<u64>>,
}

impl<'a> Container<'a> {
	/// Parse information about a container from an XML element.
	pub fn parse(elt: &'a Element) -> Result<Self> {
		if elt.name != "ItemContainer" {
			return Err(invalid_data(format!("Expected ItemContainer element but got {}", elt.name)));
		}

		let contents = elt.attributes.get("contained").ok_or_else(|| invalid_data("ItemContainer element is missing contained attribute"))?;
		let contents = contents.split(&[',', ';'][..]).map(|i| if i.is_empty() { Ok(None) } else { Ok(Some(i.parse()?)) }).collect::<std::result::Result<Vec<Option<u64>>, std::num::ParseIntError>>().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

		Ok(Self {
			xml: elt,
			contents,
		})
	}
}

/// Iterates the items in a submarine.
struct Items<'a>(std::slice::Iter<'a, XMLNode>);

impl<'a> Items<'a> {
	/// Begins iteration.
	fn new(sub: &'a Element) -> Self {
		Self(sub.children.iter())
	}
}

impl<'a> Iterator for Items<'a> {
	type Item = Result<Item<'a>>;

	fn next(&mut self) -> Option<Self::Item> {
		while let Some(i) = self.0.next() {
			if let Some(i) = i.as_element() {
				if i.name == "Item" {
					return Some(Item::parse(i))
				}
			}
		}
		None
	}
}

impl<'a> FusedIterator for Items<'a> where std::slice::Iter<'a, XMLNode>: FusedIterator {
}

/// Removes all items from all containers in a submarine.
pub fn clear_containers(filename: &OsStr, verbose: bool) -> Result<()> {
	let mut sub = load_submarine(filename)?;
	let containers = Items::new(&sub).filter(|i| if let Ok(i) = i { i.container.is_some() } else { true }).collect::<Result<Vec<Item<'_>>>>()?;
	let all_contained_items = containers.iter().flat_map(|i| &i.container.as_ref().unwrap().contents).flatten().copied().collect::<HashSet<u64>>();
	sub.children.retain(|child| if let Some(child) = child.as_element() {
		if child.name == "Item" {
			let id: u64 = child.attributes.get("ID").unwrap().parse().unwrap();
			let is_contained = all_contained_items.contains(&id);
			if verbose && is_contained {
				println!("Remove contained item ID {}", id);
			}
			!is_contained
		} else {
			true
		}
	} else {
		true
	});
	for child in sub.children.iter_mut() {
		if let Some(child) = child.as_mut_element() {
			if child.name == "Item" {
				for grandchild in child.children.iter_mut() {
					if let Some(grandchild) = grandchild.as_mut_element() {
						if grandchild.name == "ItemContainer" {
							let contained = grandchild.attributes.get_mut("contained").unwrap();
							contained.retain(|c| c == ',');
						}
					}
				}
			}
		}
	}
	save_submarine(filename, &sub)?;
	Ok(())
}

/// Lists a summary of all containers in a submarine.
pub fn list_containers(filename: &OsStr, verbose: bool) -> Result<()> {
	let sub = load_submarine(filename)?;
	let containers = Items::new(&sub).filter(|i| if let Ok(i) = i { i.container.is_some() } else { true }).collect::<Result<Vec<Item<'_>>>>()?;
	let all_contained_items = containers.iter().flat_map(|i| &i.container.as_ref().unwrap().contents).flatten().copied().collect::<HashSet<u64>>();

	println!("=== Top-level containers, by type ===");
	{
		let mut counts = BTreeMap::<&str, usize>::new();
		for container in &containers {
			if !all_contained_items.contains(&container.id) {
				*counts.entry(container.identifier).or_default() += 1;
			}
		}
		for (identifier, count) in counts.iter() {
			println!("{}: {}", identifier, count);
		}
	}

	if verbose {
		println!("=== All containers, by type ===");
		let mut counts = BTreeMap::<&str, usize>::new();
		for container in &containers {
			*counts.entry(container.identifier).or_default() += 1;
		}
		for (identifier, count) in counts.iter() {
			println!("{}: {}", identifier, count);
		}
	}

	Ok(())
}
*/
pub fn clear_containers(filename: &OsStr, verbose: bool) -> Result<()> {
	Err(std::io::Error::new(std::io::ErrorKind::Other, "Unimplemented"))
}

pub fn ident(filename: &OsStr) -> Result<()> {
	let sub = load_submarine(filename)?;
	save_submarine(filename, &sub)?;
	Ok(())
}

pub fn list_containers(filename: &OsStr, verbose: bool) -> Result<()> {
	Err(std::io::Error::new(std::io::ErrorKind::Other, "Unimplemented"))
}
