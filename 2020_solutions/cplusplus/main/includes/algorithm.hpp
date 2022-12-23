//
// Created by luker on 6/5/2021.
//

#ifndef ADVENT_OF_CODE_2020_ALGORITHM_H
#define ADVENT_OF_CODE_2020_ALGORITHM_H

#include <optional>
#include <map>

namespace Algorithm {
    template<typename R>
    class CircularQueue {
    private:
        class Node {
        public:
            Node() = default;
            R data;
            Node *next;
            Node *prev;
        };
        CircularQueue<R>::Node *current = nullptr;
        size_t m_size{0};
        std::map<R, CircularQueue<R>::Node> values;
        CircularQueue<R>::Node * findNodeWithData(const R r);
    public:
        CircularQueue() = default;

        bool isEmpty() const;

        size_t size() const;

        void enqueue(const R valueToAdd);

        std::optional<R> dequeue();

        void next();

        void prev();

        std::optional<R> currentValue() const;

        bool contains(const R r);

        void setAtValue(const R r);
    };

    template<typename R>
    bool CircularQueue<R>::isEmpty() const {
        return this->current == nullptr;
    }

    template<typename R>
    size_t CircularQueue<R>::size() const {
        return this->m_size;
    }

    template<typename R>
    void CircularQueue<R>::enqueue(const R element) {
        CircularQueue::Node *newNode;
        auto elementIter = this->values.find(element);
        if(elementIter == this->values.end()) {
            const auto node = CircularQueue::Node();
            const auto value = this->values.insert(std::pair<R, CircularQueue::Node>(element, node));

            newNode = &value.first->second;
        } else {
            newNode = &elementIter->second;
        }
        newNode->data = element;

        if (this->current == nullptr) {
            newNode->next = newNode;
            newNode->prev = newNode;
        } else {
            newNode->next = this->current;
            newNode->prev = this->current->prev;
            newNode->prev->next = newNode;
            this->current->prev = newNode;
        }
        this->m_size++;
        this->current = newNode;
    }

    template<typename R>
    std::optional<R> CircularQueue<R>::dequeue() {
        if (this->isEmpty()) {
            return {};
        }
        const auto pNode = this->current;
        const auto value = pNode->data;
        if (pNode->next == pNode) {
            this->current = nullptr;
        } else {
            this->current = pNode->next;
            this->current->prev = pNode->prev;
            pNode->prev->next = this->current;
        }

        this->values.erase(pNode->data);
        this->m_size--;

        return value;
    }

    template<typename R>
    void CircularQueue<R>::next() {
        if (this->current != nullptr) {
            this->current = this->current->next;
        }
    }

    template<typename R>
    void CircularQueue<R>::prev() {
        if (this->current != nullptr) {
            this->current = this->current->prev;
        }
    }

    template<typename R>
    std::optional<R> CircularQueue<R>::currentValue() const {
        if (this->current == nullptr) {
            return {};
        }
        return this->current->data;
    }

    template<typename R>
    bool CircularQueue<R>::contains(const R r) {
        return this->findNodeWithData(r) != nullptr;
    }

    template<typename R>
    void CircularQueue<R>::setAtValue(const R r) {
        this->current = this->findNodeWithData(r);
    }

    template<typename R>
    typename CircularQueue<R>::Node *CircularQueue<R>::findNodeWithData(const R r) {
        if (this->isEmpty()) {
            return nullptr;
        }
        auto iter = this->values.find(r);
        if (iter == this->values.end()) {
            return nullptr;
        } else {
            return &iter->second;
        }
//        auto currentNode = this->current;
//        do {
//            if(currentNode->data == r) {
//                return currentNode;
//            }
//            currentNode = currentNode-> next;
//        } while (currentNode != this->current);
//
//        return nullptr;
    }

}

#endif //ADVENT_OF_CODE_2020_ALGORITHM_H
