//
// Copyright 2023 The Matrix.org Foundation C.I.C
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

import Foundation

// MARK: - Public

/// Defines a mention type available in the Rich Text Editor.
public enum WysiwygMentionType: String {
    case user
    case room
}

public extension WysiwygMentionType {
    /// Associated pattern key.
    var patternKey: PatternKey {
        switch self {
        case .user:
            return .at
        case .room:
            return .hash
        }
    }
}

// MARK: - Internal

extension WysiwygMentionType {
    /// Default attributes.
    var attributes: [Attribute] {
        [
            Attribute(key: "data-mention-type", value: rawValue),
            Attribute(key: "contenteditable", value: "false"),
        ]
    }
}
