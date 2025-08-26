# **Software Requirements Specification (SRS)**

**Project:** RustHouse – Model Railway Collection Manager
**Version:** 1.0
**Date:** 2025-08-24
**Author:** Carlo Micieli

---

## **1. Introduction**

### **1.1 Purpose**

The purpose of **RustHouse** is to provide model railway collectors with a desktop tool to organize their collections and wish lists, record detailed information about models and rolling stock, and track purchase details. This SRS defines functional, non-functional, and data requirements to guide design and development.

### **1.2 Scope**

RustHouse will be a **GTK4-based desktop application** running on Linux and Windows. It will enable users to:

* Manage one personal collection.
* Manage multiple wish lists.
* Record detailed model and rolling stock information.
* Search, filter, and sort entries.
* Import/export data in common formats.

RustHouse **will not** include online marketplace integration, payment processing, or cloud synchronization in the initial release.

### **1.3 Definitions, Acronyms, and Abbreviations**

* **GTK4** – GIMP Toolkit, version 4 (GUI framework for Linux desktop applications)
* **Model** – A product made by a manufacturer, identified by its product code, scale, and details.
* **Rolling Stock** – Individual railway items (locomotive, freight car, passenger car) that make up a model.
* **Collection** – The set of models owned by a collector.
* **Wish List** – A named list of models the collector wants to acquire.

---

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
* At startup, the application must seed the database with a predefined list of scales and railway companies, read from a data file (e.g., `.dat` or `.json`) bundled with the application binary.
* The seeding logic must:
  * Insert a scale or railway company if it is missing from the database.
  * Do nothing if the entity exists and its version matches the seed data.
  * Update the entity if it exists but has an older version than the seed data.
* The seed data file must be maintained as part of the application source and included in all builds and distributions.
* This ensures all users have a consistent, up-to-date set of scales and railway companies, and prevents accidental user modification or deletion of these core entities.

---

## **3. Functional Requirements**

### **3.1 Data Entities**

- **Collector:** Represents the owner of the collection and wish lists. Each collector manages a single personal collection and can create multiple wish lists. Collectors can define preferences such as preferred currency, system of measure, favourite scales, favourite railway companies, and favourite eras.

- **Manufacturer:** Describes a company that produces model railway items. Includes company details, contact information, and business status. **ID format:** `urn:manufacturer:{name}` (URL-encoded).

- **Railway Model:** Represents a specific product made by a manufacturer, identified by product code, scale, descriptive details, and power method. Each model can include one or more rolling stock items. All rolling stock for a model shares the same power method, which can be one of: AC (alternate current), DC (direct current), or Trix express. Each railway model has a category, which can be one of: locomotive, freight car, passenger car, electric multiple unit, railcar, train set, or starter set. **ID format:** `urn:model:{manufacturer name}-{product code}` (URL-encoded).

- **Scale:** Defines the modeling scale (e.g., H0, N, Z) and associated properties such as ratio and track gauge. **ID format:** `urn:scale:{name}` (URL-encoded).

- **Rolling Stock:** Represents an individual railway item (locomotive, freight car, passenger car, electric multiple unit, or railcar) that is part of a model. Includes details like category, railway company, and physical attributes. The rolling stock category can be one of: locomotive, freight car, passenger car, electric multiple unit, or railcar. **ID format:** `urn:rollingstock:{model_urn}-{road_number}` (URL-encoded).

  - For **locomotives**, **railcars**, and **electric multiple units**: includes type (for locomotives: diesel, steam, electric; for railcars and EMUs: power car, trailer car), depot name, livery, series, control (no DCC, DCC ready, DCC fitted, DCC sound), and socket type (one of: NONE, NEM_355, NEM_356, NEM_357, NEM_359, NEM_360, NEM_362, NEM_365) for digital decoder. Also includes coupler properties: whether the model mounts a close coupler (`has_close_coupler`), has a standard coupler socket (`has_standard_coupler_socket`), or has a digital controller coupler (`has_digital_controller_coupler`).
  - For **passenger cars**: includes passenger car type (one of: baggage cars, combine cars, compartment coaches, dining cars, double deckers, driving trailers, lounges, observation cars, open coaches, railway post offices, sleeping cars), livery, and service level (first class, second class, third class).
  - For **freight cars**: includes freight car type (one of: auto transport cars, brake wagons, container cars, covered freight cars, deep well flat cars, dump cars, gondolas, heavy goods wagons, hinged cover wagons, hopper wagons, refrigerator cars, silo container cars, slide tarpaulin wagons, sliding wall boxcars, special transport cars, stake wagons, swing roof wagons, tank cars, telescope hood wagons) and livery.
  - For all rolling stock: optional body shell type and chassis type (allowed values: metal die cast, plastic).

- **Railway Company:** Describes a real-world railway company, including its name, country, status, and contact information. **ID format:** `urn:railway:{name}` (URL-encoded).

- **Shop:** Represents a retailer or vendor where models can be purchased or are desired. Includes contact and location details. Shops can be managed independently and marked as favourites by the user. **ID format:** `urn:shop:{name}` (URL-encoded).

- **Favourite Shops:** Represents the user's preferred shops for purchases, allowing quick access and management. Each collector can have multiple favourite shops.

- **Collection & Collection Items:** The collection is the set of models owned by the collector. Each collection item records ownership details, purchase information, and links to the model and shop.

- **Wish List & Wish List Items:** Wish lists are named lists of models the collector wants to acquire. Each wish list item records a desired model, optional target price, and shop, and is linked to a specific wish list. **Wish List ID format:** `urn:wishlist:{name}` (URL-encoded).

---

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

#### **3.2.4 Viewing & Filtering**

* Sort by manufacturer, category, scale, era, or price.
* Filter by wish list, category, or railway company.
* Search by product code or description.

#### **3.2.5 Shop Management**

* Add/Edit/Delete shops.
* View all shops in the system.
* Mark or unmark shops as favourites.

#### **3.2.6 Collector Preferences**

* Set and update preferred currency.
* Set and update preferred system of measure (mm or inches).
* Manage a list of favourite scales.
* Manage a list of favourite railway companies.
* Manage a list of favourite eras.
* Access and modify all preferences via a dedicated settings widget in the application.

#### **3.2.7 Import/Export**

* Export collection or wish lists to CSV/JSON.
* Import models from CSV (optional in v1).

---

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

---

## **5. Database Schema (Initial Proposal)**

**Tables:**

```
Collector(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    preferred_currency TEXT,           -- ISO 4217 currency code (e.g., EUR, USD)
    preferred_measure TEXT             -- 'mm' or 'inches'
)

Scale(
    id TEXT PRIMARY KEY,                -- URN: urn:scale:{name}
    name TEXT NOT NULL,           -- e.g., H0, N, Z, O, G
    ratio TEXT NOT NULL,          -- e.g., 1:87, 1:160
    track_gauge TEXT NOT NULL,    -- enum: Standard, Narrow
    gauge REAL NOT NULL,          -- mm or inches
    description TEXT,             -- optional
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

RailwayCompany(
    id TEXT PRIMARY KEY,                -- URN: urn:railway:{name}
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    status TEXT NOT NULL,            -- enum: Active, Inactive
    website_url TEXT,                -- optional
    linkedin TEXT,                   -- optional
    facebook TEXT,                   -- optional
    twitter TEXT,                    -- optional
    instagram TEXT,                  -- optional
    youtube TEXT,                    -- optional
    description TEXT,                -- optional
    created_at TEXT NOT NULL,        -- creation timestamp (ISO 8601)
    last_modified_at TEXT,           -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

Manufacturer(
    id TEXT PRIMARY KEY,                -- URN: urn:manufacturer:{name}
    name TEXT NOT NULL,
    registered_company_name TEXT, -- optional
    kind TEXT,                   -- enum: Industrial, Brass Metal Models
    status TEXT,                 -- enum: Active, Out of Business
    email TEXT,                  -- optional
    website_url TEXT,            -- optional
    phone_number TEXT,           -- optional
    street_address TEXT,         -- optional
    city TEXT,                   -- optional
    state TEXT,                  -- optional
    postal_code TEXT,            -- optional
    country TEXT,                -- optional
    linkedin TEXT,               -- optional
    facebook TEXT,               -- optional
    twitter TEXT,                -- optional
    instagram TEXT,              -- optional
    youtube TEXT,                -- optional
    created_at TEXT NOT NULL,    -- creation timestamp (ISO 8601)
    last_modified_at TEXT,       -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

RailwayModel(
    id TEXT PRIMARY KEY,                -- URN: urn:model:{manufacturer name}-{product code}
    manufacturer_id TEXT NOT NULL REFERENCES Manufacturer(id),
    product_code TEXT NOT NULL,
    description TEXT NOT NULL,
    detailed_description TEXT,    -- optional
    details TEXT,                 -- optional, rich text
    delivery_date TEXT,           -- optional, month or quarter
    delivery_state TEXT,          -- enum: Announced, Available, Cancelled, Unknown
    scale_id TEXT NOT NULL REFERENCES Scale(id),
    power_method TEXT NOT NULL,   -- enum: AC, DC, Trix express
    category TEXT NOT NULL,       -- enum: Locomotive, Freight Car, Passenger Car, Electric Multiple Unit, Railcar, Train Set, Starter Set
    -- Rolling stock is in a separate table
    UNIQUE(manufacturer_id, product_code),
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

RollingStock(
    id TEXT PRIMARY KEY, // URN: urn:rollingstock:{model_urn}-{road_number} or similar, see note below
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    category TEXT NOT NULL,       -- enum: Locomotive, Freight Car, Passenger Car, Electric Multiple Unit, Railcar
    railway_company_id TEXT NOT NULL REFERENCES RailwayCompany(id),
    length REAL NOT NULL,         -- cm/mm/in
    era TEXT NOT NULL,            -- string/enum
    road_name TEXT NOT NULL,
    road_number TEXT,             -- optional
    description TEXT,             -- optional
    details TEXT,                 -- optional, rich text
    -- Category-specific fields:
    locomotive_type TEXT,         -- enum: Diesel, Steam, Electric (locomotives only)
    depot_name TEXT,              -- locomotives, railcars, EMUs
    livery TEXT,                  -- all categories
    series TEXT,                  -- locomotives, railcars, EMUs
    control TEXT,                 -- enum: No DCC, DCC Ready, DCC Fitted, DCC Sound (locomotives, railcars, EMUs)
    socket_type TEXT,             -- enum: NONE, NEM_355, NEM_356, NEM_357, NEM_359, NEM_360, NEM_362, NEM_365 (locomotives, railcars, EMUs)
    has_close_coupler BOOLEAN,    -- true if mounts a close coupler
    has_standard_coupler_socket BOOLEAN, -- true if has a standard coupler socket
    has_digital_controller_coupler BOOLEAN, -- true if has a digital controller coupler
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
    id TEXT PRIMARY KEY,                -- URN: urn:wishlist:{name}
    name TEXT NOT NULL,                -- name of the wish list
    created_at TEXT NOT NULL,          -- creation timestamp (ISO 8601)
    last_modified_at TEXT,             -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

CollectionItem(
    id INTEGER PRIMARY KEY,
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    price REAL NOT NULL,          -- numeric, currency
    currency TEXT,                -- optional, ISO 4217 currency code (e.g., EUR, USD)
    shop_id TEXT REFERENCES Shop(id), -- optional, foreign key to Shop (URN)
    added_at TEXT NOT NULL,       -- when added to collection (ISO 8601)
    removed_at TEXT               -- when removed from collection (ISO 8601, optional)
)

WishListItem(
    id INTEGER PRIMARY KEY,
    wishlist_id TEXT NOT NULL REFERENCES WishList(id),
    model_id TEXT NOT NULL REFERENCES RailwayModel(id),
    desired_price REAL,           -- optional, currency
    currency TEXT,                -- optional, ISO 4217 currency code (e.g., EUR, USD)
    shop_id TEXT REFERENCES Shop(id), -- optional, foreign key to Shop (URN)
    wish_list_name TEXT,          -- denormalized for display, optional; must be kept in sync with WishList.name
    added_at TEXT NOT NULL,       -- when added to wishlist (ISO 8601)
    removed_at TEXT               -- when removed from wishlist (ISO 8601, optional)
)

Shop(
    id TEXT PRIMARY KEY,                -- URN: urn:shop:{name}
    name TEXT NOT NULL,
    email TEXT,                  -- optional
    website_url TEXT,            -- optional
    phone_number TEXT,           -- optional
    created_at TEXT NOT NULL,     -- creation timestamp (ISO 8601)
    last_modified_at TEXT,        -- last change timestamp (ISO 8601, optional)
    version INTEGER NOT NULL DEFAULT 1
)

FavouriteShop(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    shop_id TEXT NOT NULL REFERENCES Shop(id),
    created_at TEXT NOT NULL, -- when marked as favourite
    UNIQUE(collector_id, shop_id)
)

FavouriteScale(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    scale_id INTEGER NOT NULL REFERENCES Scale(id),
    created_at TEXT NOT NULL,
    UNIQUE(collector_id, scale_id)
)

FavouriteRailwayCompany(
    id INTEGER PRIMARY KEY,
    collector_id INTEGER NOT NULL REFERENCES Collector(id),
    railway_company_id INTEGER NOT NULL REFERENCES RailwayCompany(id),
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

---

## **6. Use Cases**

### **UC-01: Add Model to Collection**

**Actor**: Collector
**Precondition**: Model does not exist in database.
**Steps**:

1. User selects "Add Model".
2. Enters model details.
3. Adds rolling stock information.
4. Sets purchase price and shop.
5. Saves entry.

**Postcondition**: Model appears in collection list.

---

## **7. Future Enhancements**

* Photo attachment for models.
* Condition tracking.
* Cloud sync.
* Marketplace price lookup.

---

# **8. User Interface Design**

## **8.1 Main Application Layout**

```
+-------------------------------------------------------------+
| RustHouse [ File | Edit | View | Help ]                     |
+-------------------------------------------------------------+
| Sidebar (StackSwitcher)   |   Main Content (Stack)          |
|---------------------------+---------------------------------|
| [ Collection ]            |  [Collection View]              |
| [ Wish Lists ]            |                                 |
| [ Models DB ]             |                                 |
|                           |                                 |
+-------------------------------------------------------------+
| Status Bar:  Total Models: 120   |  Last Saved: 10:23 AM     |
+-------------------------------------------------------------+
```

### GTK4 Widgets:

* **Main Window** → `GtkApplicationWindow`
* **Sidebar Navigation** → `GtkStackSidebar`
* **Main Content Area** → `GtkStack` (switchable views)
* **Menu** → `GtkPopoverMenuBar`
* **Status Bar** → `GtkStatusbar`

---

## **8.2 Collection View**

```
+-------------------------------------------------------------+
| Collection                                                   |
+-------------------------------------------------------------+
| [ Search: _________ ]  [ Filter ▼ ]  [ Sort ▼ ]             |
+-------------------------------------------------------------+
| Manufacturer | Product Code | Scale | Price | Shop | Details|
|-------------------------------------------------------------|
| Märklin      | 3000         | H0    | €120  | eBay | ...    |
| Roco         | 44220        | N     | €35   | ShopX| ...    |
| ...                                                       |
+-------------------------------------------------------------+
| [ Add Model ] [ Edit ] [ Delete ] [ Export ]                |
+-------------------------------------------------------------+
```

### GTK4 Widgets:

* Toolbar: `GtkBox` with `GtkSearchEntry`, `GtkDropDown`, `GtkButton`
* Table: `GtkTreeView` or `GtkListView` with model entries
* Action buttons: `GtkButton`

---

## **8.3 Wish Lists View**

```
+-------------------------------------------------------------+
| Wish Lists                                                   |
+-------------------------------------------------------------+
| [ Wish List ▼ ]  [ New List ]  [ Delete List ]              |
+-------------------------------------------------------------+
| Manufacturer | Product Code | Scale | Desired Price | Shop  |
|-------------------------------------------------------------|
| Piko         | 57420        | H0    | €60           | ModelX|
| Roco         | 73421        | N     | €85           | ShopY |
| ...                                                       |
+-------------------------------------------------------------+
| [ Add Model ] [ Move to Collection ] [ Delete ] [ Export ]  |
+-------------------------------------------------------------+
```

### GTK4 Widgets:

* Wish List Selector: `GtkDropDown`
* Table: `GtkListView`
* Action Buttons: `GtkButton`

---

## **8.4 Model & Rolling Stock Dialog**

```
+-------------------------------------------------------------+
| Add/Edit Model                                               |
+-------------------------------------------------------------+
| Manufacturer: [__________]   Product Code: [__________]     |
| Description:  [__________________________________________] |
| Scale:        [ H0 ▼ ]                                     |
| Details:      [ Multiline TextView ]                       |
+-------------------------------------------------------------+
| Rolling Stock:                                               |
|-------------------------------------------------------------|
| Category | Railway Company | Era | Road Name | Road Number  |
|-------------------------------------------------------------|
| Loco     | DB              | IV  | BR80      | 80 020       |
| ...                                                       |
+-------------------------------------------------------------+
| [ Add Rolling Stock ] [ Remove ]                           |
+-------------------------------------------------------------+
| [ Save ] [ Cancel ]                                        |
+-------------------------------------------------------------+
```

### GTK4 Widgets:

* Form fields: `GtkEntry`, `GtkTextView`, `GtkDropDown`
* Rolling Stock Table: `GtkListView`
* Add/Remove Buttons: `GtkButton`
* Dialog container: `GtkDialog`

---

## **8.5 Navigation Flow**

1. **Startup** → Main Window opens with **Collection View** selected.
2. **Switch Sidebar** → User can switch to **Wish Lists** or **Models DB**.
3. **Add/Edit Model** → Opens dialog to define model + rolling stock.
4. **Collection/Wish List Actions** → CRUD + Import/Export buttons.
5. **Close Application** → Changes autosaved in SQLite DB.

---

## **8.6 Theming**

* Use GTK4 **Adwaita theme** for consistency.
* Dark mode support with `Adwaita-dark`.
* Consistent **icons** from [GNOME symbolic icons](https://developer.gnome.org/hig/icons).

---
