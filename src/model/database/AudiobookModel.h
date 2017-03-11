//
// Created by mistlight on 3/9/2017.
//

#ifndef NODOKA_AUDIOBOOKMODEL_H
#define NODOKA_AUDIOBOOKMODEL_H

#include "DatabaseModel.h"

namespace Database {
    class AudiobookModel : public DatabaseModel {

    public:
        void save();
        void load();

        AudiobookModel(const std::shared_ptr<Core::DatabaseInstance> &dbInstance);

    };
}


#endif //NODOKA_AUDIOBOOKMODEL_H
