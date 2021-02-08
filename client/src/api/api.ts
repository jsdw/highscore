import { client } from "./client"

// A mindless copy of api.rs from the server code.

export type LoginInput = {
    username: string
    password: string
}
export function login(opts: LoginInput): Promise<{}> {
    return client("login", opts)
}

export function logout(): Promise<{}> {
    return client("logout")
}

export type CurrentUserOutput = {
    /** Returns undefined if we aren't logged in */
    username: string | null
}
export function current_user(): Promise<CurrentUserOutput> {
    return client("current_user")
}

export type UpsertUserInput = {
    username: string
    password: string
}
export function upsert_user(opts: UpsertUserInput): Promise<{}> {
    return client("upsert_user", opts)
}

export type DeleteUserInput = {
    username: string
}
export function delete_user(opts: DeleteUserInput): Promise<{}> {
    return client("delete_user", opts)
}

export type UpsertGroupInput = {
    id?: string
    name: string
}
export type GroupOutput = {
    id: string
    name: string
}
export function upsert_group(opts: UpsertGroupInput): Promise<GroupOutput> {
    return client("upsert_group", opts)
}

export type DeleteGroupInput = {
    id: string
}
export function delete_group(opts: DeleteGroupInput): Promise<{}> {
    return client("delete_group", opts)
}

export type GetGroupInput = {
    id: string
}
export function get_group(opts: GetGroupInput): Promise<GroupOutput> {
    return client("get_group", opts)
}

export type UpsertScorableInput = {
    id?: string
    group_id: string
    name: string
}
export type ScorableOutput = {
    id: string
    name: string
}
export function upsert_scorable(opts: UpsertScorableInput): Promise<ScorableOutput> {
    return client("upsert_scorable", opts)
}

export type GetScorableInput = {
    id: string
}
export function get_scorable(opts: GetScorableInput): Promise<ScorableOutput> {
    return client("get_scorable", opts)
}

export type DeleteScorableInput = {
    id: string
}
export function delete_scorable(opts: DeleteScorableInput): Promise<{}> {
    return client("delete_scorable", opts)
}

export type UpsertScoreInput = {
    id?: string
    scorable_id: string
    username: string
    value: number
    /** ISO date string */
    date?: string
}
export type UpsertScoreOutput = {
    id: string
}
export function upsert_score(opts: UpsertScoreInput): Promise<UpsertScoreOutput> {
    return client("upsert_score", opts)
}

export type DeleteScoreInput = {
    id: string
}
export function delete_score(opts: DeleteScoreInput): Promise<{}> {
    return client("delete_score", opts)
}

export type GroupsOutput = Group[]
export type Group = {
    id: string
    name: string
}
export function groups(): Promise<GroupsOutput> {
    return client("groups")
}

export type ScorablesInGroupInput = {
    group_id: string
}
export type ScorablesInGroupOutput = Scorable[]
export type Scorable = {
    id: string
    name: string
}
export function scorables_in_group(opts: ScorablesInGroupInput): Promise<ScorablesInGroupOutput> {
    return client("scorables_in_group", opts)
}

export type ScoresInput = {
    scorable_id: string
    limit?: number
}
export type ScoresOutput = Score[]
export type Score = {
    id: string
    username: string
    value: number
    /** ISO date string */
    date: string
}
export function scores(opts: ScoresInput): Promise<ScoresOutput> {
    return client("scores", opts)
}