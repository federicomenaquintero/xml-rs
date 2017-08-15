// Reference to an external entity's data
pub enum ExternalId {
    System(String), // URI, no "#" (fragment identifier) accepted
    Public {
        public_id: String,
        system_id: String // URI, no "#" (fragment identifier) accepted
    }
};

// Usable in the document body
pub enum OwnedGeneralEntity {
    Internal {
        name: String;
        value: String;
    },

    External {
        name: String;
        external_id: ExternalId,
        notation: String // ndata
    }
};

// Usable in the DTD
pub enum OwnedParsedEntity {
    Internal {
        name: String;
        value: String;
    },

    External {
        name: String;
        external_id: ExternalId
    }
};
