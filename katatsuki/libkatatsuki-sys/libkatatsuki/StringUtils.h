#pragma once

#include <algorithm>
#include <cctype>
#include <locale>
#include "taglib/tstring.h"
#include "taglib/tstringlist.h"


TagLib::String join(const TagLib::StringList & v, const std::string & delimiter = ",") {
    TagLib::String out;
    if (auto i = v.begin(), e = v.end(); i != e) {
        out += *i++;
        for (; i != e; ++i) out.append(delimiter).append(*i);
    }
    return out;
}
