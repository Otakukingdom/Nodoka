//
// Created by mistlight on 1/8/17.
//

#include "MediaProperty.h"

MediaProperty::MediaProperty() {
    this->isNull = true;
}

MediaProperty::MediaProperty(long long mediaDuration) {
    this->isNull = false;

    this->duration = mediaDuration;
}

bool MediaProperty::isNullObject() {
    return this->isNull;
}

long long MediaProperty::getDuration() {
    return this->duration;
}
