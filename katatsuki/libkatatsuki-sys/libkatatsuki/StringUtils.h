#pragma once

#include <algorithm>
#include <cctype>
#include <locale>
#include <tstring.h>
#include <tstringlist.h>


TagLib::String join(const TagLib::StringList & v, const std::string & delimiter = ",") {
    TagLib::String out;
    if (auto i = v.begin(), e = v.end(); i != e) {
        out += *i++;
        for (; i != e; ++i) out.append(delimiter).append(*i);
    }
    return out;
}
