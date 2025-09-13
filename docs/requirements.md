# **APP: Requirements**

## **1. Introduction**

### **1.1 Purpose**

The purpose of **RustHouse** is to provide model railway collectors with a desktop tool to organize their collections and wish lists, record detailed information about models and rolling stock, and track purchase details.

### **1.2 Scope**

RustHouse will be a **GTK4-based desktop application** running on Linux and Windows. It will enable users to:

* Manage one personal collection.
* Manage multiple wish lists.
* Record detailed model and rolling stock information.
* Track purchase information for items in the collection.
* Search, filter, and sort entries.
* Import/export data in common formats.

RustHouse **will not** include online marketplace integration, payment processing, or cloud synchronization in the initial release.

### **1.3 Definitions, Acronyms, and Abbreviations**

* **GTK4** – GIMP Toolkit, version 4 (GUI framework for Linux desktop applications)
* **Model** – A product made by a manufacturer, identified by its product code, scale, and details.
* **Rolling Stock** – Individual railway items (locomotive, freight car, passenger car) that make up a model.
* **Collection** – The set of models owned by a collector.
* **Wish List** – A named list of models the collector wants to acquire.
* **Railway Company** – The real-world railway operator or company that the model or rolling stock represents (e.g., DB, SNCF, Amtrak).
* **Scale** – The proportional ratio between the model's size and the real-world object it represents (e.g., HO, N, O, Z).
* **Set** – A group of models or rolling stock items packaged and sold together as a single product (e.g., starter sets, train packs).

## **2. Overall Description**

### **2.1 Product Perspective**

RustHouse is a **standalone desktop application** with local data storage using SQLite. The application’s main view will allow quick switching between the **Collection** and **Wish Lists**.

### **2.2 User Characteristics**

* **Primary User**: Model railway collector.
* Skills: Basic desktop software usage and knowledge of model railway terminology.
* Expected data volume: 50–5,000 models.

### **2.3 Constraints**

* Runs on Linux and Windows with the GTK4 runtime installed.
* Local storage only (no online sync in v1).
* All data in UTF-8 encoding.
* The application must be packaged for distribution using Flatpak or Snap on Linux, and as a standalone installer or portable executable on Windows to ensure easy installation and sandboxing.

### **2.4 Assumptions and Dependencies**

* The user has GTK4 installed on their system.
* SQLite database file is stored in the user’s home directory (e.g., `~/.local/share/rusthouse/rusthouse.sqlite` on Linux, `%USERPROFILE%\\AppData\\Local\\rusthouse\\rusthouse.sqlite` on Windows).
* Optional CSV import/export requires basic knowledge of CSV files.

### **2.5 Data Seeding for Scales and Railway Companies**

* The application must not require the user to manually create or edit scales or railway companies.
* At startup, the application must seed the database with a predefined list of scales and railway companies, read from a data set in the **Avro** format bundled with the application binary.
* The seeding logic must:
  * Insert a scale or railway company if it is missing from the database.
  * Do nothing if the entity exists and its version matches the seed data.
  * Update the entity if it exists but has an older version than the seed data.
* The seed data file must be maintained as part of the application source and included in all builds and distributions.

#### **2.5.1 Advantages of using Avro over text-based formats (e.g., JSON)**
  * Avro is a compact, binary format, resulting in smaller file sizes and faster read/write operations.
  * Avro enforces a strict schema, ensuring data consistency and validation at both read and write time.
  * Schema evolution is supported, allowing the data structure to change over time without breaking compatibility.
  * Binary encoding reduces parsing errors and improves performance compared to text-based formats.
  * Avro is widely supported in data processing tools and languages, making integration and future automation easier.
* This ensures all users have a consistent, up-to-date set of scales and railway companies, and prevents accidental user modification or deletion of these core entities.

## **3. Functional Requirements**

### **3.1 Data Entities**

- **Manufacturer:** Describes a company that produces model railway items. Includes company details, contact information, and business status. **ID format:** `urn:manufacturer:{name}` (URL-encoded).

- **Model (Railway Model):** Represents a specific product made by a manufacturer, identified by product code, scale, descriptive details, and power method. Main properties: `id`, `manufacturer_id` (FK), `product_code`, `description`, `scale_id` (FK), `power_method` (AC, DC, Trix express), `category` (e.g., Locomotive, Freight Car, Passenger Car, Electric Multiple Unit, Railcar), `delivery_date`, `delivery_state`, `details`, `created_at`, `last_modified_at`. Each model can include one or more rolling stock items. All rolling stock for a model shares the same power method. **ID format:** `urn:model:{manufacturer name}-{product code}` (URL-encoded).

- **Set:** Represents a group of models or rolling stock items packaged and sold together as a single product (e.g., starter sets, train packs). Main properties: `id`, `manufacturer_id` (FK), `catalogue_number`, `name`, `scale`, `release_year`, `status`. The contents of a set are defined by the `SetContents` entity, which links each set to its constituent models and their quantities.

- **Scale:** Defines the modeling scale (e.g., H0, N, Z) and associated properties such as ratio and track gauge. **ID format:** `urn:scale:{name}` (URL-encoded).

- **Rolling Stock:** Represents an individual railway item (locomotive, freight car, passenger car, electric multiple unit, or railcar) that is part of a model. Includes details like category, railway company, and physical attributes. The rolling stock category can be one of: locomotive, freight car, passenger car, electric multiple unit, or railcar. **ID format:** `urn:rollingstock:{model_urn}-{road_number}` (URL-encoded).

  - For **locomotives**, **railcars**, and **electric multiple units**: includes type (for locomotives: diesel, steam, electric; for railcars and EMUs: power car, trailer car), depot name, livery, series, control (no DCC, DCC ready, DCC fitted, DCC sound), and socket type (one of: NONE, NEM_355, NEM_356, NEM_357, NEM_359, NEM_360, NEM_362, NEM_365) for digital decoder. Also includes coupler properties: whether the model mounts a close coupler (`has_close_coupler`), has a standard coupler socket (`has_standard_coupler_socket`), or has a digital controller coupler (`has_digital_controller_coupler`).
  - For **passenger cars**: includes passenger car type (one of: baggage cars, combine cars, compartment coaches, dining cars, double deckers, driving trailers, lounges, observation cars, open coaches, railway post offices, sleeping cars), livery, and service level (first class, second class, third class).
  - For **freight cars**: includes freight car type (one of: auto transport cars, brake wagons, container cars, covered freight cars, deep well flat cars, dump cars, gondolas, heavy goods wagons, hinged cover wagons, hopper wagons, refrigerator cars, silo container cars, slide tarpaulin wagons, sliding wall boxcars, special transport cars, stake wagons, swing roof wagons, tank cars, telescope hood wagons) and livery.
  - For all rolling stock: optional body shell type and chassis type (allowed values: metal die cast, plastic).

- **Railway Company:** Describes a real-world railway company, including its name, country, status, and contact information. **ID format:** `urn:railway:{name}` (URL-encoded).

- **Seller:** Represents an entity from which a model or set can be purchased. A seller can be either a Shop (retailer/vendor) or a Private Collector. Each seller has a type (`shop` or `collector`), a unique ID, and relevant contact/location details. This allows tracking purchases from both shops and private individuals. **ID format:** `urn:seller:{type}:{name}` (URL-encoded).

- **Favourite Shops:** Represents the user's preferred shops for purchases, allowing quick access and management. Each collector can have multiple favourite shops.

- **Favourite Scales:** Links a collector to their preferred modeling scales. Each record associates a collector with a scale (e.g., H0, N, Z). Main properties: `id`, `collector_id` (FK), `scale_id` (FK), `created_at`.

- **Favourite Railway Companies:** Links a collector to their preferred railway companies. Each record associates a collector with a railway company (e.g., DB, SNCF, Amtrak). Main properties: `id`, `collector_id` (FK), `railway_company_id` (FK), `created_at`.

- **Collector:** Represents the owner of the collection and wish lists. Each collector manages a single personal collection and can create multiple wish lists. Collectors can define preferences such as preferred currency, system of measure, favourite scales, favourite railway companies, and favourite eras.

- **Collection & Collection Items:** The collection is the set of models and sets owned by the collector. Each collection item records ownership details, purchase information, and links to either a model or a set, as well as the shop. Main properties: `id`, `collection_id` (FK), `item_type` (Model or Set), `item_id` (FK to Model or Set), `purchase_date`, `purchase_price`, `currency`, `vendor`, `condition`, `notes`, `quantity`.

- **Wish List & Wish List Items:** Wish lists are named lists of models and sets the collector wants to acquire. Each wish list item records a desired model or set, optional target price, and shop, and is linked to a specific wish list. Main properties: `id`, `wishlist_id` (FK), `item_type` (Model or Set), `item_id` (FK to Model or Set), `target_price`, `shop_id` (optional), `notes`. **Wish List ID format:** `urn:wishlist:{name}` (URL-encoded).

### **3.2 Features**

#### **3.2.1 Model & Rolling Stock Management**

* Create/Edit/Delete models.
* Add one or more rolling stocks per model.
* Manage rolling stock attributes.

#### **3.2.2 Collection Management**

* Add/Edit/Delete collection models.
* Record purchase details (price, shop).
* Search and filter collection entries.
* Manage a list of favourite shops for quick access when adding or editing collection items.

#### **3.2.3 Wish List Management**

* Create/Delete wish lists.
* Add/Edit/Delete wish list items.
* Move items from wish list to collection.

#### **3.2.4 Rolling Stocks Management**

* View a list of all rolling stock items owned by the collector, across all models and sets
* For each rolling stock item, display key technical information, including:
  * Whether a digital decoder is installed (and type, if available).
  * Maintenance status or reminders if the item is due for maintenance.
* Filter and search the depot by category, scale, or technical attributes.
* Quickly identify which items need attention or upgrades.

#### **3.2.5 Viewing & Filtering**

* Sort by manufacturer, category, scale, era, or price.
* Filter by wish list, category, or railway company.
* Search by product code or description.

#### **3.2.6 Shop Management**

* Add/Edit/Delete shops.
* View all shops in the system.
* Mark or unmark shops as favourites.

#### **3.2.7 Collector Preferences**

* Set and update preferred currency.
* Set and update preferred system of measure (mm or inches).
* Manage a list of favourite scales.
* Manage a list of favourite railway companies.
* Manage a list of favourite eras.
* Access and modify all preferences via a dedicated settings widget in the application.

#### **3.2.8 Import/Export**

* Export collection or wish lists to CSV/JSON.
* Import models from CSV (optional in v1).

## **4. Non-Functional Requirements**

### **4.1 Performance**

* Search/filter results in under 1 second for up to 5,000 models.

### **4.2 Reliability**

* Local database integrity checks on startup.
* Auto-save on data modification.

### **4.3 Usability**

* GTK4-based interface with clear navigation.
* Keyboard shortcuts for common actions.

### **4.4 Maintainability**

* Modular code structure for adding new attributes.
* Separation between UI and data logic.

### **4.5 Security**

* User data stored locally in SQLite file with file system permissions.

## **5. User Stories**

### **5.1 Collection Management**
* As a collector
  I want to add models and sets to my collection
  So that I can keep track of what I own and their purchase details.
* As a collector
  I want to edit or remove items from my collection
  So that my collection stays accurate and up to date.
* As a collector
  I want to search and filter my collection by manufacturer, category, scale, or price
  So that I can quickly find specific items.

### **5.2 Wish List Management**
* As a collector
  I want to create and manage wish lists of models and sets
  So that I can plan future purchases and track desired items.
* As a collector
  I want to move items from a wish list to my collection
  So that I can record when I acquire something I wanted.

### **5.3 Model & Rolling Stock Management**
* As a collector
  I want to add, edit, or remove models and their rolling stock
  So that my collection reflects all the details of each item.

### **5.4 Depot Management**
* As a collector
  I want to view a list of all rolling stock I own in one place
  So that I can easily see and manage my entire inventory.
* As a collector
  I want to see technical information for each rolling stock item, such as decoder installation and maintenance status
  So that I can keep my collection in good working order and plan upgrades or repairs.
* As a collector
  I want to filter or search by technical attributes (e.g., decoder type, maintenance needed)
  So that I can quickly find items that need attention or have specific features.

### **5.5 Shop Management**
* As a collector
  I want to add, edit, or remove shops
  So that I can track where I buy or want to buy items.
* As a collector
  I want to mark shops as favourites
  So that I can quickly select them when adding collection or wish list items.

### **5.6 Preferences**
* As a collector
  I want to set my preferred currency, measurement system, favourite scales, railway companies, and eras
  So that the app matches my personal collecting interests.

### **5.7 Import/Export**
* As a collector
  I want to export my collection or wish lists to CSV or JSON
  So that I can back up or share my data.
* As a collector
  I want to import models from CSV
  So that I can quickly add many items at once.

## **6. Database Schema**

### **6.1 Schema**

```sql
Collector(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    preferred_currency TEXT,           -- ISO 4217 currency code (e.g., EUR, USD)
    preferred_measure TEXT             -- 'mm' or 'inches'
)

Scale(
  id TEXT PRIMARY KEY,                  -- URN: urn:scale:{name}
    name TEXT NOT NULL,                 -- e.g., H0, N, Z, O, G
    ratio TEXT NOT NULL,                -- e.g., 1:87, 1:160
    track_gauge TEXT NOT NULL,          -- enum: Standard, Narrow
    gauge REAL NOT NULL,                -- mm or inches
    description TEXT,                   -- optional
    created_at TEXT NOT NULL,           -- creation timestamp (ISO 8601)
    last_modified_at TEXT,              -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

RailwayCompany(
    id TEXT PRIMARY KEY,                -- URN: urn:railway:{name}
    name TEXT NOT NULL,
    registered_company_name TEXT,       -- optional
    country TEXT NOT NULL,              -- ISO 3166-1 alpha-2 country code
    status TEXT NOT NULL,               -- enum: Active, Inactive
    website_url TEXT,                   -- optional
    linkedin TEXT,                      -- optional
    facebook TEXT,                      -- optional
    twitter TEXT,                       -- optional
    instagram TEXT,                     -- optional
    youtube TEXT,                       -- optional
    description TEXT,                   -- optional
    created_at TEXT NOT NULL,           -- creation timestamp (ISO 8601)
    last_modified_at TEXT,              -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

Manufacturer(
    id TEXT PRIMARY KEY,                -- URN: urn:manufacturer:{name}
    name TEXT NOT NULL,
    registered_company_name TEXT,       -- optional
    kind TEXT,                          -- enum: Industrial, Brass Metal Models
    status TEXT,                        -- enum: Active, Out of Business
    email TEXT,                         -- optional
    website_url TEXT,                   -- optional
    phone_number TEXT,                  -- optional
    street_address TEXT,                -- optional
    city TEXT,                          -- optional
    state TEXT,                         -- optional
    postal_code TEXT,                   -- optional
    country TEXT,                       -- optional, ISO 3166-1 alpha-2 country code
    linkedin TEXT,                      -- optional
    facebook TEXT,                      -- optional
    twitter TEXT,                       -- optional
    instagram TEXT,                     -- optional
    youtube TEXT,                       -- optional
    created_at TEXT NOT NULL,           -- creation timestamp (ISO 8601)
    last_modified_at TEXT,              -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

RailwayModel(
    id TEXT PRIMARY KEY,                -- URN: urn:model:{manufacturer name}-{product code}
    manufacturer_id TEXT NOT NULL REFERENCES Manufacturer(id),
    product_code TEXT NOT NULL,
    description TEXT NOT NULL,
    detailed_description TEXT,    -- optional, rich text
    delivery_date TEXT,           -- optional, month or quarter
    delivery_state TEXT,          -- enum: Announced, Available, Cancelled, Unknown
    scale_id TEXT NOT NULL REFERENCES Scale(id),    
    power_method TEXT NOT NULL,   -- enum: AC, DC, Trix express
    category TEXT NOT NULL,       -- enum: Locomotive, Freight Car, Passenger Car, Electric Multiple Unit, Railcar, Train Set, Starter Set
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
    UNIQUE(manufacturer_id, product_code),
)

RollingStock(
    id TEXT PRIMARY KEY,          -- URN: urn:rollingstock:{model_urn}-{road_number} or similar, see note below
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    category TEXT NOT NULL,       -- enum: Locomotive, Freight Car, Passenger Car, Electric Multiple Unit, Railcar
    railway_company_id TEXT NOT NULL REFERENCES RailwayCompany(id),
    length REAL NOT NULL,         -- cm/mm/in
    era TEXT NOT NULL,            -- string/enum
    road_name TEXT NOT NULL,
    road_number TEXT,             -- optional
    description TEXT,             -- optional
    detailed_description TEXT,    -- optional, rich text
    -- Category-specific fields:
    locomotive_type TEXT,         -- enum: Diesel, Steam, Electric (locomotives only)
    depot_name TEXT,              -- locomotives, railcars, EMUs
    livery TEXT,                  -- all categories
    series TEXT,                  -- locomotives, railcars, EMUs
    control TEXT,                 -- enum: No DCC, DCC Ready, DCC Fitted, DCC Sound (locomotives, railcars, EMUs)
    dcc_socket_type TEXT,             -- enum (locomotives, railcars, EMUs)
    coupler_socket_type TEXT,     -- enum: NEM_355, NEM_356, NEM_357, NEM_358, NEM_359, NEM_360, NEM_361, NEM_362, NEM_363, NEM_365
    has_close_coupler BOOLEAN,    -- true if mounts a close coupler
    has_standard_coupler_socket BOOLEAN, -- true if has a standard coupler socket
    has_digital_controller_coupler BOOLEAN, -- true if has a digital controller coupler
    min_radius REAL,              -- optional
    railcar_type TEXT,            -- enum: Power Car, Trailer Car (railcars only)
    emu_type TEXT,                -- enum: Power Car, Trailer Car (EMUs only)
    passenger_car_type TEXT,      -- passenger cars only
    service_level TEXT,           -- enum: First Class, Second Class, Third Class (passenger cars only)
    freight_car_type TEXT,        -- freight cars only
    body_shell_type TEXT,         -- enum: metal die cast, plastic (optional, all categories)
    chassis_type TEXT,            -- enum: metal die cast, plastic (optional, all categories)
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

WishList(
    id TEXT PRIMARY KEY,               -- URN: urn:wishlist:{name}
    name TEXT NOT NULL,                -- name of the wish list
    description TEXT,                  -- optional
    created_at TEXT NOT NULL,          -- creation timestamp (ISO 8601)
    last_modified_at TEXT,             -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

CollectionItem(
    id INTEGER PRIMARY KEY,
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    price REAL NOT NULL,          -- numeric, currency
    currency TEXT,                -- optional, ISO 4217 currency code (e.g., EUR, USD)
    seller_id TEXT REFERENCES Seller(id), -- optional, foreign key to Seller (URN)
    added_at TEXT NOT NULL,       -- when added to collection (ISO 8601)
    removed_at TEXT               -- when removed from collection (ISO 8601, optional)
)

WishListItem(
    id INTEGER PRIMARY KEY,
    wishlist_id TEXT NOT NULL REFERENCES WishList(id),
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    desired_price REAL,           -- optional, currency
    currency TEXT,                -- optional, ISO 4217 currency code (e.g., EUR, USD)
    seller_id TEXT REFERENCES Seller(id), -- optional, foreign key to Seller (URN)
    priority TEXT NOT NULL DEFAULT 'NORMAL', -- enum: HIGH, NORMAL, LOW
    added_at TEXT NOT NULL,       -- when added to wishlist (ISO 8601)
    removed_at TEXT               -- when removed from wishlist (ISO 8601, optional)
)

Seller(
    id TEXT PRIMARY KEY,         -- URN: urn:seller:{type}:{name}
    name TEXT NOT NULL,
    type TEXT,                   -- enum: shop, collector
    email TEXT,                  -- optional
    website_url TEXT,            -- optional
    phone_number TEXT,           -- optional
    street_address TEXT,         -- optional
    city TEXT,                   -- optional
    state TEXT,                  -- optional
    postal_code TEXT,            -- optional
    country TEXT,                -- optional, ISO 3166-1 alpha-2 country code
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

FavouriteSeller(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    seller_id TEXT NOT NULL REFERENCES Seller(id),
    created_at TEXT NOT NULL, -- when marked as favourite
    UNIQUE(collector_id, seller_id)
)

FavouriteScale(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    scale_id TEXT NOT NULL REFERENCES Scale(id),
    created_at TEXT NOT NULL,
    UNIQUE(collector_id, scale_id)
)

FavouriteRailwayCompany(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    railway_company_id TEXT NOT NULL REFERENCES RailwayCompany(id),
    created_at TEXT NOT NULL,
    UNIQUE(collector_id, railway_company_id)
)

FavouriteEra(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    era TEXT NOT NULL, -- string/enum, e.g., 'III', 'IV', 'V'
    created_at TEXT NOT NULL,
    UNIQUE(collector_id, era)
)

```

### 6.1 Enumerations

#### 6.1.1 Seller Type
* Name: Seller Type
* Description: The type of seller entity from which a model or set can be purchased.
* Values:
  * `SHOP` – Retailer or commercial seller
  * `COLLECTOR` – Private individual or hobbyist

#### 6.1.2 Track Gauge
* Name: Track Gauge
* Description: The type of track gauge for a scale.
* Values:
  * `STANDARD` – Standard gauge track
  * `NARROW` – Narrow gauge track

#### 6.1.3 Railway Company Status
* Name: Railway Company Status
* Description: The operational status of a railway company.
* Values:
  * `ACTIVE` – Currently operating
  * `INACTIVE` – No longer operating

#### 6.1.4 Manufacturer Kind
* Name: Manufacturer Kind
* Description: The kind of manufacturer.
* Values:
  * `INDUSTRIAL` – Mass-market manufacturer
  * `BRASS_METAL_MODELS` – Specialist, often hand-built models

#### 6.1.5 Manufacturer Status
* Name: Manufacturer Status
* Description: The operational status of a manufacturer.
* Values:
  * `ACTIVE` – Company is in business
  * `OUT_OF_BUSINESS` – Company has ceased operations

#### 6.1.6 Delivery State
* Name: Delivery State
* Description: The delivery state of a railway model.
* Values:
  * `ANNOUNCED` – Model announced, not yet available
  * `AVAILABLE` – Model is available for purchase
  * `CANCELLED` – Model was cancelled
  * `UNKNOWN` – Status not specified

#### 6.1.7 Power Method
* Name: Power Method
* Description: The power method used by a railway model.
* Values:
  * `AC` – Alternating current power
  * `DC` – Direct current power
  * `TRIX_EXPRESS` – Trix Express system

#### 6.1.8 Model Category
* Name: Model Category
* Description: The category of a railway model.
* Values:
  * `LOCOMOTIVE` – Self-propelled engine
  * `FREIGHT_CAR` – Car for goods or cargo
  * `PASSENGER_CAR` – Car for passengers
  * `ELECTRIC_MULTIPLE_UNIT` – Self-propelled electric trainset
  * `RAILCAR` – Self-propelled single car
  * `TRAIN_SET` – Boxed set of multiple cars/locos
  * `STARTER_SET` – Beginner's set with track and controller

#### 6.1.9 Rolling Stock Category
* Name: Rolling Stock Category
* Description: The category of a rolling stock item.
* Values:
  * `LOCOMOTIVE` – Self-propelled engine
  * `FREIGHT_CAR` – Car for goods or cargo
  * `PASSENGER_CAR` – Car for passengers
  * `ELECTRIC_MULTIPLE_UNIT` – Self-propelled electric trainset
  * `RAILCAR` – Self-propelled single car

#### 6.1.10 Locomotive Type
* Name: Locomotive Type
* Description: The type of locomotive.
* Values:
  * `DIESEL_LOCOMOTIVE` – Powered by diesel engine
  * `STEAM_LOCOMOTIVE` – Powered by steam engine
  * `ELECTRIC_LOCOMOTIVE` – Powered by electric motor
  * `SHUNTING_LOCOMOTIVE` – For yard/switching duties

#### 6.1.11 Railcar
* Name: Railcar Type
* Description: The type of railcar.
* Values:
  * `POWER_CAR` – Motorized car in a trainset
  * `TRAILER_CAR` – Non-powered car in a trainset

#### 6.1.12 EMU Type
* Name: EMU Type
* Description: The type of electric multiple unit.
* Values:
  * `DRIVING_CAR` – Car with driver's cab, controls train but may not be powered
  * `HIGH_SPEED_TRAIN` – EMU designed for high-speed service
  * `MOTOR_CAR` – Powered car with traction motors
  * `POWER_CAR` – Main powered car in the EMU
  * `TRAILER_CAR` – Non-powered car, no traction motors
  * `TRAIN_SET` – Complete EMU set, may include multiple car types

#### 6.1.13 Control
* Name: Control
* Description: The digital control capability of a model.
* Values:
  * `NO_DCC` – No digital decoder, analog only
  * `DCC_READY` – Prepared for DCC, socket for decoder
  * `DCC_FITTED` – Digital decoder installed
  * `DCC_SOUND` – Digital decoder with sound functions

#### 6.1.14 Socket Type
* Name: Socket Type
* Description: The type of NEM digital decoder socket for DCC or digital control, as per NEM standards.
* Values:
  * `NEM_651` – 6-pin, small scale
  * `NEM_652` – 8-pin, standard
  * `NEM_654` – 21-pin, PluX
  * `NEM_658` – 22-pin, PluX22
  * `NEM_660` – 21MTC
  * `NEXT18` – Next18 socket
  * `WIRE` – Hardwired
  * `NONE` – No socket

#### 6.1.15 Passenger Car Type
* Name: Passenger Car Type
* Description: The type of passenger car.
* Values:
  * `BAGGAGE_CAR` – Car for luggage and parcels
  * `COMBINE_CAR` – Car combining passenger and baggage sections
  * `COMPARTMENT_COACH` – Coach with individual compartments
  * `DINING_CAR` – Car with restaurant or dining facilities
  * `DOUBLE_DECKER` – Two-level passenger car
  * `DRIVING_TRAILER` – Passenger car with driver's cab (no engine)
  * `LOUNGE` – Car with lounge or observation seating
  * `OBSERVATION_CAR` – Car with panoramic windows, often at train end
  * `OPEN_COACH` – Coach with open seating (no compartments)
  * `RAILWAY_POST_OFFICE` – Car for mail sorting and transport
  * `SLEEPING_CAR` – Car with beds or sleeping compartments

#### 6.1.16 Service Level
* Name: Service Level
* Description: The service level of a passenger car.
* Values:
  * `FIRST_CLASS` – Premium passenger accommodation
  * `MIXED_FIRST_SECOND_CLASS` – Both first and second class
  * `SECOND_CLASS` – Standard passenger accommodation
  * `MIXED_SECOND_THIRD_CLASS` – Both second and third class
  * `THIRD_CLASS` – Basic passenger accommodation

#### 6.1.17 Freight Car Type
* Name: Freight Car Type
* Description: The type of freight car.
* Values:
  * `AUTO_TRANSPORT_CAR` – For transporting automobiles
  * `BRAKE_WAGON` – Equipped with handbrake, often for train end
  * `CONTAINER_CAR` – Carries shipping containers
  * `COVERED_FREIGHT_CAR` – Enclosed car for general goods
  * `DEEP_WELL_FLAT_CAR` – Low-floor car for tall/large loads
  * `DUMP_CAR` – For bulk materials, can tip to unload
  * `GONDOLA` – Open-topped car for bulk goods
  * `HEAVY_GOODS_WAGON` – For very heavy or oversized cargo
  * `HINGED_COVER_WAGON` – Covered car with hinged roof for loading
  * `HOPPER_WAGON` – For bulk goods, unloads from bottom
  * `REFRIGERATOR_CAR` – Insulated, for perishable goods
  * `SILO_CONTAINER_CAR` – For powders or granules, with silo containers
  * `SLIDE_TARPAULIN_WAGON` – Covered with sliding tarpaulin for easy access
  * `SLIDING_WALL_BOXCAR` – Boxcar with sliding walls for loading
  * `SPECIAL_TRANSPORT_CAR` – For special or unusual loads
  * `STAKE_WAGON` – Flat car with stakes for logs or pipes
  * `SWING_ROOF_WAGON` – Roof swings open for loading/unloading
  * `TANK_CAR` – For liquids or gases
  * `TELESCOPE_HOOD_WAGON` – Covered car with telescoping hood for coils or sheet metal

#### 6.1.19 Body Shell
* Name: Body Shell
* Description: The material type for body shell.
* Values:
  * `METAL_DIE_CAST` – Made from metal die casting
  * `PLASTIC` – Made from plastic

#### 6.1.20 Chassis Type
* Name: Chassis Type
* Description: The material type for chassis.
* Values:
  * `METAL_DIE_CAST` – Made from metal die casting
  * `PLASTIC` – Made from plastic

#### 6.1.21 Priority
* Name: Priority
* Description: The priority of a wish list item.
* Values:
  * `HIGH` – Highest priority
  * `NORMAL` – Normal priority
  * `LOW` – Lowest priority

#### 6.1.22 Epoch
* Name: Epoch
* Description: The historical railway era or epoch classification for rolling stock and models.
* Values:
  * `I` – Early railways (approx. 1835–1920)
  * `II` – Grouping and nationalization (approx. 1920–1945)
  * `IIa` – Early part of Epoch II
  * `IIb` – Later part of Epoch II
  * `III` – Postwar, steam/diesel transition (approx. 1945–1970)
  * `IIIa` – Early part of Epoch III
  * `IIIb` – Later part of Epoch III
  * `IV` – Modernization, UIC numbering (approx. 1970–1990)
  * `IVa` – Early part of Epoch IV
  * `IVb` – Later part of Epoch IV
  * `V` – Privatization, new liveries (approx. 1990–2006)
  * `Vm` – Modern sub-epoch of V
  * `VI` – Contemporary era (approx. 2007–present)

#### 6.1.23 Coupler Socket Type
* Name: Coupler Socket Type
* Description: The type of coupler socket, following NEM standards for model railway couplers.
* Values:
  * `NEM_355` – Coupler pocket for Z scale (1:220)
  * `NEM_356` – Coupler pocket for N scale (1:160)
  * `NEM_357` – Coupler pocket for TT scale (1:120)
  * `NEM_358` – Coupler pocket for H0e/H0m narrow gauge
  * `NEM_359` – Coupler pocket for H0 scale (1:87), standard
  * `NEM_360` – Coupler pocket for O scale (1:45)
  * `NEM_361` – Coupler pocket for 1 scale (1:32)
  * `NEM_362` – Universal close coupler pocket (widely used in H0)
  * `NEM_363` – Coupler pocket for G scale (1:22.5)
  * `NEM_365` – Coupler pocket for narrow gauge and special applications